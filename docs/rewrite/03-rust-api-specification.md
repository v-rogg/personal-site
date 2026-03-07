# Rust API Specification

## Overview

Single Axum-based service handling all backend logic, replacing both Cloudflare Workers.

## Dependencies (Cargo.toml)

```toml
[package]
name = "vr-www-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "compression-gzip"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Utilities
uuid = { version = "1", features = ["v4"] }
petname = "2"
base64 = "0.21"
flate2 = "1"           # gzip compression
chrono = { version = "0.4", features = ["serde"] }

# HTTP client (for Friendly Captcha verification)
reqwest = { version = "0.11", features = ["json"] }

# Email (SMTP)
lettre = { version = "0.11", features = ["tokio1-native-tls"] }

# GeoIP lookup
maxminddb = "0.24"

# Environment & config
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Error handling
thiserror = "1"
anyhow = "1"
```

---

## Project Structure

```
backend/
├── Cargo.toml
├── Dockerfile
├── migrations/
│   └── 001_init.sql
└── src/
    ├── main.rs              # Entry point, server setup
    ├── config.rs            # Environment configuration
    ├── error.rs             # Error types and handling
    ├── db/
    │   ├── mod.rs           # Database pool setup
    │   └── migrations.rs    # Migration runner
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   ├── session.rs
    │   ├── signature.rs
    │   └── event.rs
    ├── routes/
    │   ├── mod.rs           # Router setup
    │   ├── sessions.rs      # POST /api/sessions
    │   ├── events.rs        # POST /api/events
    │   ├── signatures.rs    # CRUD /api/signatures
    │   ├── contact.rs       # POST /api/contact
    │   └── stats.rs         # GET /api/stats
    ├── services/
    │   ├── mod.rs
    │   ├── email.rs         # SMTP (configurable provider)
    │   └── captcha.rs       # Friendly Captcha (Germany)
    └── cache/
        └── mod.rs           # In-memory signature cache
```

---

## Configuration (config.rs)

```rust
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    
    // SMTP (configurable - Resend, mailbox.org, or any SMTP provider)
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    
    // Friendly Captcha (German provider)
    pub friendly_captcha_secret: String,
    pub friendly_captcha_sitekey: String,
    
    // Email settings
    pub email_from: String,
    pub email_to: String,
    
    // App settings
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "/data/app.db".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            smtp_host: env::var("SMTP_HOST")
                .expect("SMTP_HOST required"),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(465),
            smtp_user: env::var("SMTP_USER")
                .expect("SMTP_USER required"),
            smtp_pass: env::var("SMTP_PASS")
                .expect("SMTP_PASS required"),
            friendly_captcha_secret: env::var("FRIENDLY_CAPTCHA_SECRET")
                .expect("FRIENDLY_CAPTCHA_SECRET required"),
            friendly_captcha_sitekey: env::var("FRIENDLY_CAPTCHA_SITEKEY")
                .expect("FRIENDLY_CAPTCHA_SITEKEY required"),
            email_from: env::var("EMAIL_FROM")
                .unwrap_or_else(|_| "noreply@valentinrogg.de".to_string()),
            email_to: env::var("EMAIL_TO")
                .unwrap_or_else(|_| "mail@valentinrogg.de".to_string()),
            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "https://valentinrogg.de".to_string()),
        }
    }
}
```

---

## Application State

```rust
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::Signature;
use crate::config::Config;

pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub signature_cache: RwLock<Vec<CachedSignature>>,
}

#[derive(Clone)]
pub struct CachedSignature {
    pub id: String,
    pub name: String,
    pub signature: String,  // Already decompressed for fast serving
    pub ts_created: i64,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Arc<Self>> {
        let db = SqlitePool::connect(&config.database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&db).await?;
        
        // Load approved signatures into cache
        let signatures = Self::load_signatures(&db).await?;
        
        Ok(Arc::new(Self {
            db,
            config,
            signature_cache: RwLock::new(signatures),
        }))
    }
    
    async fn load_signatures(db: &SqlitePool) -> anyhow::Result<Vec<CachedSignature>> {
        let rows = sqlx::query_as!(
            CachedSignature,
            r#"
            SELECT id, name, signature, ts_created
            FROM signatures
            WHERE approved = 1
            ORDER BY ts_created DESC
            "#
        )
        .fetch_all(db)
        .await?;
        
        Ok(rows)
    }
    
    pub async fn refresh_cache(&self) -> anyhow::Result<()> {
        let signatures = Self::load_signatures(&self.db).await?;
        let mut cache = self.signature_cache.write().await;
        *cache = signatures;
        Ok(())
    }
}
```

---

## API Endpoints

### POST /api/sessions

Create or update a session on first event.

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response:** `201 Created`
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "created": true
}
```

**Implementation:**
```rust
pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    let now = chrono::Utc::now().timestamp();
    
    let result = sqlx::query!(
        r#"
        INSERT INTO sessions (id, ts_created, ts_last_seen)
        VALUES (?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET ts_last_seen = excluded.ts_last_seen
        "#,
        req.session_id,
        now,
        now
    )
    .execute(&state.db)
    .await?;
    
    Ok(Json(CreateSessionResponse {
        session_id: req.session_id,
        created: result.rows_affected() > 0,
    }))
}
```

---

### POST /api/events

Track an event for a session.

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "event_name": "editor.open",
    "properties": {}
}
```

**Response:** `201 Created`
```json
{
    "event_id": 12345
}
```

**Implementation:**
```rust
pub async fn create_event(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<CreateEventResponse>, AppError> {
    let now = chrono::Utc::now().timestamp();
    
    // Ensure session exists
    sqlx::query!(
        r#"
        INSERT INTO sessions (id, ts_created, ts_last_seen)
        VALUES (?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET ts_last_seen = excluded.ts_last_seen
        "#,
        req.session_id,
        now,
        now
    )
    .execute(&state.db)
    .await?;
    
    // Insert event
    let properties = req.properties.map(|p| serde_json::to_string(&p).unwrap());
    
    let result = sqlx::query!(
        r#"
        INSERT INTO events (session_id, event_name, timestamp, properties)
        VALUES (?, ?, ?, ?)
        "#,
        req.session_id,
        req.event_name,
        now,
        properties
    )
    .execute(&state.db)
    .await?;
    
    Ok(Json(CreateEventResponse {
        event_id: result.last_insert_rowid(),
    }))
}
```

---

### GET /api/signatures

List all approved signatures.

**Response:** `200 OK`
```json
{
    "signatures": [
        {
            "id": "happy-blue-cat",
            "name": "John Doe",
            "ts_created": 1702814161
        }
    ]
}
```

**Implementation:**
```rust
pub async fn list_signatures(
    State(state): State<Arc<AppState>>,
) -> Json<ListSignaturesResponse> {
    let cache = state.signature_cache.read().await;
    
    Json(ListSignaturesResponse {
        signatures: cache.iter().map(|s| SignatureSummary {
            id: s.id.clone(),
            name: s.name.clone(),
            ts_created: s.ts_created,
        }).collect(),
    })
}
```

---

### GET /api/signatures/:id

Get a single signature with full drawing data.

**Response:** `200 OK`
```json
{
    "id": "happy-blue-cat",
    "name": "John Doe",
    "signature": "<base64 drawing data>",
    "ts_created": 1702814161
}
```

**Implementation:**
```rust
pub async fn get_signature(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<SignatureResponse>, AppError> {
    // Try cache first
    let cache = state.signature_cache.read().await;
    if let Some(sig) = cache.iter().find(|s| s.id == id) {
        return Ok(Json(SignatureResponse {
            id: sig.id.clone(),
            name: sig.name.clone(),
            signature: sig.signature.clone(),
            ts_created: sig.ts_created,
        }));
    }
    drop(cache);
    
    // Fallback to DB (for unapproved signatures, admin access)
    let sig = sqlx::query_as!(
        SignatureRow,
        "SELECT id, name, signature, ts_created FROM signatures WHERE id = ?",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(SignatureResponse {
        id: sig.id,
        name: sig.name,
        signature: sig.signature,
        ts_created: sig.ts_created,
    }))
}
```

---

### POST /api/signatures

Create a new signature.

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "signature": "<base64 gzip drawing data>"
}
```

**Response:** `201 Created`
```json
{
    "id": "happy-blue-cat",
    "url": "https://valentinrogg.de/?s=happy-blue-cat"
}
```

**Implementation:**
```rust
pub async fn create_signature(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSignatureRequest>,
) -> Result<Json<CreateSignatureResponse>, AppError> {
    let now = chrono::Utc::now().timestamp();
    
    // Generate unique petname ID
    let id = generate_unique_petname(&state.db).await?;
    
    // Create or get user if email provided
    let user_id = if let Some(email) = &req.email {
        Some(get_or_create_user(&state.db, email, now).await?)
    } else {
        None
    };
    
    // Link session to user
    if let Some(uid) = user_id {
        sqlx::query!(
            "UPDATE sessions SET user_id = ? WHERE id = ?",
            uid,
            req.session_id
        )
        .execute(&state.db)
        .await?;
    }
    
    // Insert signature
    sqlx::query!(
        r#"
        INSERT INTO signatures (id, session_id, name, signature, approved, ts_created)
        VALUES (?, ?, ?, ?, 0, ?)
        "#,
        id,
        req.session_id,
        req.name,
        req.signature,
        now
    )
    .execute(&state.db)
    .await?;
    
    // Send confirmation email
    let url = format!("{}/?s={}", state.config.base_url, id);
    state.services.email.send_signature_confirmation(
        &req.name,
        &id,
        &url,
        req.email.as_deref(),
    ).await?;
    
    Ok(Json(CreateSignatureResponse { id, url }))
}

async fn generate_unique_petname(db: &SqlitePool) -> anyhow::Result<String> {
    loop {
        let id = petname::petname(3, "-");
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM signatures WHERE id = ?)",
            id
        )
        .fetch_one(db)
        .await?;
        
        if exists == 0 {
            return Ok(id);
        }
    }
}

async fn get_or_create_user(db: &SqlitePool, email: &str, now: i64) -> anyhow::Result<i64> {
    let result = sqlx::query_scalar!(
        r#"
        INSERT INTO users (email, ts_created) VALUES (?, ?)
        ON CONFLICT(email) DO UPDATE SET email = email
        RETURNING id
        "#,
        email,
        now
    )
    .fetch_one(db)
    .await?;
    
    Ok(result)
}
```

---

### POST /api/contact

Submit contact form (work request).

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "john@example.com",
    "message": "I'd like to work with you on...",
    "captcha_solution": "<friendly captcha solution>"
}
```

**Response:** `200 OK`
```json
{
    "success": true
}
```

**Implementation:**
```rust
pub async fn submit_contact(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    // Validate CAPTCHA (Friendly Captcha)
    state.services.captcha.verify(&req.captcha_solution).await?;
    
    let now = chrono::Utc::now().timestamp();
    
    // Create or get user
    let user_id = get_or_create_user(&state.db, &req.email, now).await?;
    
    // Link session to user
    sqlx::query!(
        "UPDATE sessions SET user_id = ? WHERE id = ?",
        user_id,
        req.session_id
    )
    .execute(&state.db)
    .await?;
    
    // Track event
    sqlx::query!(
        "INSERT INTO events (session_id, event_name, timestamp) VALUES (?, 'contact.submit', ?)",
        req.session_id,
        now
    )
    .execute(&state.db)
    .await?;
    
    // Send email
    state.services.email.send_contact_request(&req.email, &req.message).await?;
    
    Ok(Json(ContactResponse { success: true }))
}
```

---

### GET /api/stats

Get aggregated analytics.

**Query Parameters:**
- `days` (optional, default: 180): Time range in days

**Response:** `200 OK`
```json
{
    "conversion_step_1": 150,
    "conversion_step_2": 45,
    "conversion_rate": 30.0,
    "drawing_durations": [
        {
            "id": "happy-blue-cat",
            "duration_seconds": 342,
            "ts_created": 1702814161
        }
    ],
    "has_drawing_data": true,
    "average_eraser_uses": 2.5,
    "average_drawing_time_seconds": 120
}
```

**Implementation:**
```rust
pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    Query(params): Query<StatsParams>,
) -> Result<Json<StatsResponse>, AppError> {
    let days = params.days.unwrap_or(180);
    let since = chrono::Utc::now().timestamp() - (days * 86400);
    
    // Conversion rate
    let conversion = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT CASE WHEN event_name = 'editor.open' THEN session_id END) as step_1,
            COUNT(DISTINCT CASE WHEN event_name = 'editor.save' THEN session_id END) as step_2
        FROM events
        WHERE timestamp > ?
        "#,
        since
    )
    .fetch_one(&state.db)
    .await?;
    
    let step_1 = conversion.step_1.unwrap_or(0) as i64;
    let step_2 = conversion.step_2.unwrap_or(0) as i64;
    let rate = if step_1 > 0 { (step_2 as f64 / step_1 as f64) * 100.0 } else { 0.0 };
    
    // Drawing durations
    let durations = sqlx::query_as!(
        DrawingDuration,
        r#"
        SELECT 
            sig.id,
            MIN(e_save.timestamp - e_open.timestamp) as duration_seconds,
            sig.ts_created
        FROM signatures sig
        JOIN events e_open ON sig.session_id = e_open.session_id AND e_open.event_name = 'editor.open'
        JOIN events e_save ON sig.session_id = e_save.session_id AND e_save.event_name = 'editor.save'
        WHERE e_save.timestamp > e_open.timestamp
        GROUP BY sig.id
        ORDER BY duration_seconds DESC
        "#
    )
    .fetch_all(&state.db)
    .await?;
    
    // Average drawing time
    let avg_time: f64 = if !durations.is_empty() {
        durations.iter().map(|d| d.duration_seconds as f64).sum::<f64>() / durations.len() as f64
    } else {
        0.0
    };
    
    // Average eraser uses
    let eraser = sqlx::query!(
        r#"
        SELECT AVG(cnt) as avg FROM (
            SELECT COUNT(*) as cnt
            FROM events
            WHERE event_name = 'editor.eraser' AND timestamp > ?
            GROUP BY session_id
        )
        "#,
        since
    )
    .fetch_one(&state.db)
    .await?;
    
    Ok(Json(StatsResponse {
        conversion_step_1: step_1,
        conversion_step_2: step_2,
        conversion_rate: rate,
        drawing_durations: durations,
        has_drawing_data: !durations.is_empty(),
        average_eraser_uses: eraser.avg.unwrap_or(0.0),
        average_drawing_time_seconds: avg_time,
    }))
}
```

---

### POST /api/page-views

Track a page view. Called when user lands on a page.

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "path": "/",
    "referrer": "https://google.com"
}
```

**Response:** `201 Created`
```json
{
    "page_view_id": 12345
}
```

**Implementation:**
```rust
pub async fn create_page_view(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CreatePageViewRequest>,
) -> Result<Json<CreatePageViewResponse>, AppError> {
    let now = chrono::Utc::now().timestamp();
    
    // Ensure session exists and get/set geo data
    let (country_code, region) = lookup_geo(&state.geoip_reader, addr.ip());
    
    sqlx::query!(
        r#"
        INSERT INTO sessions (id, country_code, region, ts_created, ts_last_seen)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET 
            ts_last_seen = excluded.ts_last_seen,
            country_code = COALESCE(sessions.country_code, excluded.country_code),
            region = COALESCE(sessions.region, excluded.region)
        "#,
        req.session_id,
        country_code,
        region,
        now,
        now
    )
    .execute(&state.db)
    .await?;
    
    // Only store external referrers
    let referrer = req.referrer.filter(|r| !r.contains("valentinrogg.de"));
    
    // Insert page view
    let result = sqlx::query!(
        r#"
        INSERT INTO page_views (session_id, path, referrer, ts_start, max_scroll_percent)
        VALUES (?, ?, ?, ?, 0)
        "#,
        req.session_id,
        req.path,
        referrer,
        now
    )
    .execute(&state.db)
    .await?;
    
    Ok(Json(CreatePageViewResponse {
        page_view_id: result.last_insert_rowid(),
    }))
}

fn lookup_geo(reader: &maxminddb::Reader<Vec<u8>>, ip: IpAddr) -> (Option<String>, Option<String>) {
    if let Ok(city) = reader.lookup::<maxminddb::geoip2::City>(ip) {
        let country = city.country
            .and_then(|c| c.iso_code)
            .map(|s| s.to_string());
        let region = city.subdivisions
            .and_then(|s| s.first())
            .and_then(|s| s.names.as_ref())
            .and_then(|n| n.get("en"))
            .map(|s| s.to_string());
        (country, region)
    } else {
        (None, None)
    }
}
```

---

### PATCH /api/page-views/:id

Update page view with engagement data. Called on scroll or page unload.

**Request:**
```json
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "max_scroll_percent": 75,
    "ended": true
}
```

**Response:** `200 OK`
```json
{
    "success": true
}
```

**Implementation:**
```rust
pub async fn update_page_view(
    State(state): State<Arc<AppState>>,
    Path(page_view_id): Path<i64>,
    Json(req): Json<UpdatePageViewRequest>,
) -> Result<Json<UpdatePageViewResponse>, AppError> {
    let now = chrono::Utc::now().timestamp();
    
    if req.ended {
        // Page unload - calculate duration
        sqlx::query!(
            r#"
            UPDATE page_views 
            SET ts_end = ?,
                duration_seconds = ? - ts_start,
                max_scroll_percent = MAX(max_scroll_percent, ?)
            WHERE id = ? AND session_id = ?
            "#,
            now,
            now,
            req.max_scroll_percent.unwrap_or(0),
            page_view_id,
            req.session_id
        )
        .execute(&state.db)
        .await?;
    } else if let Some(scroll) = req.max_scroll_percent {
        // Scroll update - only update if higher
        sqlx::query!(
            r#"
            UPDATE page_views 
            SET max_scroll_percent = MAX(max_scroll_percent, ?)
            WHERE id = ? AND session_id = ?
            "#,
            scroll,
            page_view_id,
            req.session_id
        )
        .execute(&state.db)
        .await?;
    }
    
    // Update session last seen
    sqlx::query!(
        "UPDATE sessions SET ts_last_seen = ? WHERE id = ?",
        now,
        req.session_id
    )
    .execute(&state.db)
    .await?;
    
    Ok(Json(UpdatePageViewResponse { success: true }))
}
```

---

### GET /api/analytics

Get comprehensive analytics dashboard data.

**Query Parameters:**
- `days` (optional, default: 30): Time range in days

**Response:** `200 OK`
```json
{
    "summary": {
        "total_sessions": 1234,
        "total_page_views": 5678,
        "unique_visitors": 987,
        "avg_session_duration_seconds": 145,
        "avg_scroll_depth_percent": 62,
        "bounce_rate_percent": 35.5
    },
    "top_pages": [
        { "path": "/", "views": 3000, "unique_visitors": 800, "avg_duration": 120, "avg_scroll": 70 }
    ],
    "top_referrers": [
        { "referrer": "https://google.com", "visits": 500, "unique_sessions": 450 }
    ],
    "countries": [
        { "country_code": "DE", "sessions": 600 },
        { "country_code": "US", "sessions": 200 }
    ],
    "daily_stats": [
        { "date": "2024-01-15", "sessions": 50, "page_views": 120 }
    ],
    "signature_funnel": {
        "visited_home": 1000,
        "opened_editor": 150,
        "saved_signature": 45
    }
}
```

**Implementation:**
```rust
pub async fn get_analytics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AnalyticsParams>,
) -> Result<Json<AnalyticsResponse>, AppError> {
    let days = params.days.unwrap_or(30);
    let since = chrono::Utc::now().timestamp() - (days as i64 * 86400);
    
    // Summary stats
    let summary = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT s.id) as total_sessions,
            COUNT(pv.id) as total_page_views,
            COUNT(DISTINCT s.user_id) as identified_users,
            AVG(pv.duration_seconds) as avg_duration,
            AVG(pv.max_scroll_percent) as avg_scroll
        FROM sessions s
        LEFT JOIN page_views pv ON s.id = pv.session_id
        WHERE s.ts_created > ?
        "#,
        since
    )
    .fetch_one(&state.db)
    .await?;
    
    // Bounce rate
    let bounce = sqlx::query!(
        r#"
        SELECT 
            ROUND(
                COUNT(DISTINCT CASE WHEN page_count = 1 THEN session_id END) * 100.0 /
                NULLIF(COUNT(DISTINCT session_id), 0),
                1
            ) as bounce_rate
        FROM (
            SELECT session_id, COUNT(*) as page_count
            FROM page_views
            WHERE ts_start > ?
            GROUP BY session_id
        )
        "#,
        since
    )
    .fetch_one(&state.db)
    .await?;
    
    // Top pages
    let top_pages = sqlx::query_as!(
        TopPage,
        r#"
        SELECT 
            path,
            COUNT(*) as views,
            COUNT(DISTINCT session_id) as unique_visitors,
            AVG(duration_seconds) as avg_duration,
            AVG(max_scroll_percent) as avg_scroll
        FROM page_views
        WHERE ts_start > ?
        GROUP BY path
        ORDER BY views DESC
        LIMIT 10
        "#,
        since
    )
    .fetch_all(&state.db)
    .await?;
    
    // Top referrers
    let top_referrers = sqlx::query_as!(
        TopReferrer,
        r#"
        SELECT 
            referrer,
            COUNT(*) as visits,
            COUNT(DISTINCT session_id) as unique_sessions
        FROM page_views
        WHERE ts_start > ? AND referrer IS NOT NULL AND referrer != ''
        GROUP BY referrer
        ORDER BY visits DESC
        LIMIT 10
        "#,
        since
    )
    .fetch_all(&state.db)
    .await?;
    
    // Countries
    let countries = sqlx::query_as!(
        CountryStats,
        r#"
        SELECT country_code, COUNT(DISTINCT id) as sessions
        FROM sessions
        WHERE ts_created > ? AND country_code IS NOT NULL
        GROUP BY country_code
        ORDER BY sessions DESC
        LIMIT 20
        "#,
        since
    )
    .fetch_all(&state.db)
    .await?;
    
    // Daily stats
    let daily_stats = sqlx::query_as!(
        DailyStats,
        r#"
        SELECT 
            date(s.ts_created, 'unixepoch') as date,
            COUNT(DISTINCT s.id) as sessions,
            COUNT(pv.id) as page_views
        FROM sessions s
        LEFT JOIN page_views pv ON s.id = pv.session_id
        WHERE s.ts_created > ?
        GROUP BY date
        ORDER BY date DESC
        "#,
        since
    )
    .fetch_all(&state.db)
    .await?;
    
    // Signature funnel
    let funnel = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT pv.session_id) as visited_home,
            COUNT(DISTINCT CASE WHEN e_open.session_id IS NOT NULL THEN pv.session_id END) as opened_editor,
            COUNT(DISTINCT CASE WHEN e_save.session_id IS NOT NULL THEN pv.session_id END) as saved_signature
        FROM page_views pv
        LEFT JOIN events e_open ON pv.session_id = e_open.session_id AND e_open.event_name = 'editor.open'
        LEFT JOIN events e_save ON pv.session_id = e_save.session_id AND e_save.event_name = 'editor.save'
        WHERE pv.path = '/' AND pv.ts_start > ?
        "#,
        since
    )
    .fetch_one(&state.db)
    .await?;
    
    Ok(Json(AnalyticsResponse {
        summary: AnalyticsSummary {
            total_sessions: summary.total_sessions.unwrap_or(0) as i64,
            total_page_views: summary.total_page_views.unwrap_or(0) as i64,
            unique_visitors: summary.identified_users.unwrap_or(0) as i64,
            avg_session_duration_seconds: summary.avg_duration.unwrap_or(0.0),
            avg_scroll_depth_percent: summary.avg_scroll.unwrap_or(0.0),
            bounce_rate_percent: bounce.bounce_rate.unwrap_or(0.0),
        },
        top_pages,
        top_referrers,
        countries,
        daily_stats,
        signature_funnel: SignatureFunnel {
            visited_home: funnel.visited_home.unwrap_or(0) as i64,
            opened_editor: funnel.opened_editor.unwrap_or(0) as i64,
            saved_signature: funnel.saved_signature.unwrap_or(0) as i64,
        },
    }))
}
```

---

## Services

### Email Service (SMTP)

Uses `lettre` for SMTP email sending. Compatible with any SMTP provider:
- **Resend** (current): API-based, easy setup
- **mailbox.org**: German provider, GDPR-compliant, ~3 EUR/month
- **Any SMTP server**: Self-hosted or third-party

```rust
// src/services/email.rs

use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl EmailService {
    pub fn new(
        smtp_host: &str,
        smtp_port: u16,
        smtp_user: &str,
        smtp_pass: &str,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Self> {
        let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());
        
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();
        
        Ok(Self {
            mailer,
            from: from.parse()?,
            to: to.parse()?,
        })
    }
    
    pub async fn send_signature_confirmation(
        &self,
        name: &str,
        id: &str,
        url: &str,
        reply_to: Option<&str>,
    ) -> anyhow::Result<()> {
        let subject = format!("Neue Signatur: {}", name);
        let body = format!(
            r#"<html>
            <body>
                <p>Neue Signatur von <strong>{}</strong></p>
                <p>ID: {}</p>
                <p>Link: <a href="{}">{}</a></p>
            </body>
            </html>"#,
            name, id, url, url
        );
        
        let mut builder = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject(subject)
            .header(ContentType::TEXT_HTML);
        
        if let Some(rt) = reply_to {
            builder = builder.reply_to(rt.parse()?);
        }
        
        let email = builder.body(body)?;
        self.mailer.send(email).await?;
        
        Ok(())
    }
    
    pub async fn send_contact_request(
        &self,
        from_email: &str,
        message: &str,
    ) -> anyhow::Result<()> {
        let body = format!(
            r#"<html>
            <body>
                <p>Von: <strong>{}</strong></p>
                <p>{}</p>
            </body>
            </html>"#,
            from_email,
            message.replace('\n', "<br>")
        );
        
        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .reply_to(from_email.parse()?)
            .subject("Neue Arbeitsanfrage")
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        
        self.mailer.send(email).await?;
        
        Ok(())
    }
}
```

### CAPTCHA Service (Friendly Captcha - Germany)

[Friendly Captcha](https://friendlycaptcha.com/) is a German privacy-focused CAPTCHA provider.

```rust
// src/services/captcha.rs

use serde::Deserialize;

pub struct CaptchaService {
    client: reqwest::Client,
    secret: String,
    sitekey: String,
}

#[derive(Deserialize)]
struct FriendlyCaptchaResponse {
    success: bool,
    #[serde(default)]
    errors: Vec<String>,
}

impl CaptchaService {
    pub fn new(secret: String, sitekey: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            secret,
            sitekey,
        }
    }
    
    pub async fn verify(&self, solution: &str) -> Result<(), AppError> {
        let response: FriendlyCaptchaResponse = self.client
            .post("https://api.friendlycaptcha.com/api/v1/siteverify")
            .json(&serde_json::json!({
                "solution": solution,
                "secret": self.secret,
                "sitekey": self.sitekey,
            }))
            .send()
            .await?
            .json()
            .await?;
        
        if response.success {
            Ok(())
        } else {
            tracing::warn!("Friendly Captcha verification failed: {:?}", response.errors);
            Err(AppError::CaptchaFailed)
        }
    }
}
```

### Frontend Integration (Friendly Captcha)

```html
<!-- Add to your HTML -->
<script type="module" src="https://cdn.jsdelivr.net/npm/friendly-challenge@0.9.14/widget.module.min.js" async defer></script>

<!-- In your form -->
<form id="contact-form">
    <input type="email" name="email" required>
    <textarea name="message" required></textarea>
    
    <!-- Friendly Captcha widget -->
    <div class="frc-captcha" data-sitekey="YOUR_SITEKEY"></div>
    
    <button type="submit">Send</button>
</form>

<script>
    document.getElementById('contact-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const form = e.target;
        const solution = form.querySelector('[name="frc-captcha-solution"]').value;
        
        await fetch('/api/contact', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                session_id: getSessionId(),
                email: form.email.value,
                message: form.message.value,
                captcha_solution: solution,  // Send solution to backend
            }),
        });
    });
</script>
```

### Svelte Component Example

```svelte
<!-- FriendlyCaptcha.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    
    export let sitekey: string;
    export let solution = '';
    
    let container: HTMLDivElement;
    let widget: any;
    
    onMount(async () => {
        const { WidgetInstance } = await import('friendly-challenge');
        
        widget = new WidgetInstance(container, {
            sitekey,
            doneCallback: (sol: string) => {
                solution = sol;
            },
            errorCallback: (err: Error) => {
                console.error('Captcha error:', err);
            },
        });
        
        return () => widget?.destroy();
    });
    
    export function reset() {
        widget?.reset();
        solution = '';
    }
</script>

<div bind:this={container}></div>
```

Usage in form:
```svelte
<script>
    import FriendlyCaptcha from './FriendlyCaptcha.svelte';
    
    let captchaSolution = '';
    let captchaComponent: FriendlyCaptcha;
    
    async function handleSubmit() {
        if (!captchaSolution) {
            alert('Please complete the CAPTCHA');
            return;
        }
        
        await fetch('/api/contact', {
            method: 'POST',
            body: JSON.stringify({
                // ... other fields
                captcha_solution: captchaSolution,
            }),
        });
        
        captchaComponent.reset();
    }
</script>

<form on:submit|preventDefault={handleSubmit}>
    <!-- form fields -->
    <FriendlyCaptcha 
        bind:this={captchaComponent}
        sitekey="YOUR_SITEKEY" 
        bind:solution={captchaSolution} 
    />
    <button type="submit">Send</button>
</form>
```

---

## Error Handling

```rust
// src/error.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    
    #[error("CAPTCHA validation failed")]
    CaptchaFailed,
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::CaptchaFailed => (StatusCode::BAD_REQUEST, "CAPTCHA validation failed"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Request(_) => (StatusCode::BAD_GATEWAY, "External service error"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        
        tracing::error!("Error: {:?}", self);
        
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
```

---

## Main Entry Point

```rust
// src/main.rs

use axum::{routing::{get, post, patch}, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::sync::Arc;

mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod cache;

use config::Config;
use routes::{sessions, events, signatures, contact, stats, page_views, analytics};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("vr_www_api=debug".parse()?)
        )
        .init();
    
    // Load config
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    
    // Initialize state (includes GeoIP reader)
    let state = AppState::new(config.clone()).await?;
    
    // Build router
    let app = Router::new()
        // Session & event tracking
        .route("/api/sessions", post(sessions::create_session))
        .route("/api/events", post(events::create_event))
        // Page view tracking (enhanced analytics)
        .route("/api/page-views", post(page_views::create_page_view))
        .route("/api/page-views/:id", patch(page_views::update_page_view))
        // Signatures
        .route("/api/signatures", get(signatures::list_signatures))
        .route("/api/signatures", post(signatures::create_signature))
        .route("/api/signatures/:id", get(signatures::get_signature))
        // Contact form
        .route("/api/contact", post(contact::submit_contact))
        // Analytics
        .route("/api/stats", get(stats::get_stats))
        .route("/api/analytics", get(analytics::get_analytics))
        // Health check
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    
    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

---

## GeoIP Setup

Download the free GeoLite2 City database from MaxMind (requires free account):

```bash
# Download and extract
wget "https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key=YOUR_KEY&suffix=tar.gz" -O geolite2.tar.gz
tar -xzf geolite2.tar.gz
mv GeoLite2-City_*/GeoLite2-City.mmdb /data/GeoLite2-City.mmdb
```

Add to AppState:

```rust
use maxminddb::Reader;

pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub signature_cache: RwLock<Vec<CachedSignature>>,
    pub geoip_reader: Reader<Vec<u8>>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Arc<Self>> {
        let db = SqlitePool::connect(&config.database_url).await?;
        sqlx::migrate!("./migrations").run(&db).await?;
        
        let signatures = Self::load_signatures(&db).await?;
        
        // Load GeoIP database
        let geoip_reader = Reader::open_readfile(&config.geoip_path)?;
        
        Ok(Arc::new(Self {
            db,
            config,
            signature_cache: RwLock::new(signatures),
            geoip_reader,
        }))
    }
}
```

Add to config:

```rust
pub struct Config {
    // ... existing fields ...
    pub geoip_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            // ... existing fields ...
            geoip_path: env::var("GEOIP_PATH")
                .unwrap_or_else(|_| "/data/GeoLite2-City.mmdb".to_string()),
        }
    }
}
```

Add to Dockerfile:

```dockerfile
# Download GeoIP database during build (or mount as volume)
RUN wget -q "https://download.maxmind.com/..." -O /data/GeoLite2-City.mmdb
```

Or mount as a volume in docker-compose:

```yaml
api:
  volumes:
    - ./data:/data
    - ./GeoLite2-City.mmdb:/data/GeoLite2-City.mmdb:ro
```
