use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub async fn create_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // Ensure the database directory exists
    if let Some(parent) = Path::new(database_url).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}?mode=rwc", database_url))
        .await?;

    // Enable WAL mode and foreign keys
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA synchronous = NORMAL")
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    // Create migrations tracking table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            applied_at INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Run migrations in order
    let migrations = [
        ("001_init", include_str!("../../migrations/001_init.sql")),
    ];

    for (name, sql) in migrations {
        // Check if already applied
        let applied: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM _migrations WHERE name = ?")
                .bind(name)
                .fetch_optional(pool)
                .await?;

        if applied.is_some() {
            tracing::debug!("Migration {} already applied, skipping", name);
            continue;
        }

        tracing::info!("Running migration: {}", name);

        // Split by semicolon and execute each statement
        for statement in sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty()
                && !statement.starts_with("--")
                && !statement.starts_with("PRAGMA")
            {
                if let Err(e) = sqlx::query(statement).execute(pool).await {
                    tracing::error!("Migration {} failed on statement: {}", name, statement);
                    return Err(e.into());
                }
            }
        }

        // Record migration
        let now = chrono::Utc::now().timestamp_millis();
        sqlx::query("INSERT INTO _migrations (name, applied_at) VALUES (?, ?)")
            .bind(name)
            .bind(now)
            .execute(pool)
            .await?;

        tracing::info!("Migration {} completed", name);
    }

    tracing::info!("Database migrations completed");
    Ok(())
}
