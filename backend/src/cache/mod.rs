use crate::models::CachedSignature;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

pub struct SignatureCache {
    signatures: RwLock<Vec<CachedSignature>>,
}

impl SignatureCache {
    pub fn new() -> Self {
        Self {
            signatures: RwLock::new(Vec::new()),
        }
    }

    pub async fn load(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let signatures = sqlx::query_as::<_, CachedSignature>(
            r#"
            SELECT id, name, signature, ts_created
            FROM signatures
            WHERE approved = 1
            ORDER BY ts_created DESC
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut cache = self.signatures.write().await;
        *cache = signatures;
        tracing::info!("Loaded {} approved signatures into cache", cache.len());
        Ok(())
    }

    pub async fn get_all(&self) -> Vec<CachedSignature> {
        self.signatures.read().await.clone()
    }

    pub async fn get_by_id(&self, id: &str) -> Option<CachedSignature> {
        self.signatures
            .read()
            .await
            .iter()
            .find(|s| s.id == id)
            .cloned()
    }

    pub async fn refresh(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        self.load(pool).await
    }

    /// Check if a signature is in the cache
    pub async fn contains(&self, id: &str) -> bool {
        self.signatures.read().await.iter().any(|s| s.id == id)
    }

    /// Add a single signature to the cache, maintaining descending ts_created order
    pub async fn add(&self, signature: CachedSignature) {
        let mut cache = self.signatures.write().await;
        // Find insertion position to maintain descending order by ts_created
        let pos = cache
            .iter()
            .position(|s| s.ts_created < signature.ts_created)
            .unwrap_or(cache.len());
        let id = signature.id.clone();
        cache.insert(pos, signature);
        tracing::debug!("Added signature {} to cache at position {}", id, pos);
    }

    /// Remove a signature from the cache
    pub async fn remove(&self, id: &str) -> bool {
        let mut cache = self.signatures.write().await;
        if let Some(pos) = cache.iter().position(|s| s.id == id) {
            cache.remove(pos);
            tracing::debug!("Removed signature {} from cache", id);
            true
        } else {
            false
        }
    }

    /// Update a signature's fields in the cache
    pub async fn update(&self, id: &str, name: Option<String>, signature: Option<String>) -> bool {
        let mut cache = self.signatures.write().await;
        if let Some(cached) = cache.iter_mut().find(|s| s.id == id) {
            if let Some(new_name) = name {
                cached.name = new_name;
            }
            if let Some(new_sig) = signature {
                cached.signature = new_sig;
            }
            tracing::debug!("Updated signature {} in cache", id);
            true
        } else {
            false
        }
    }

    /// Fetch a signature from DB and add it to cache
    pub async fn add_from_db(&self, pool: &SqlitePool, id: &str) -> anyhow::Result<bool> {
        let signature = sqlx::query_as::<_, CachedSignature>(
            r#"
            SELECT id, name, signature, ts_created
            FROM signatures
            WHERE id = ? AND approved = 1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        if let Some(sig) = signature {
            self.add(sig).await;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for SignatureCache {
    fn default() -> Self {
        Self::new()
    }
}
