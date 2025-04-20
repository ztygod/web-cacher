use crate::fetcher::FetchResult;
use anyhow::{anyhow, Result};
use chrono::Duration;
use lru::LruCache;
use std::{num::NonZeroUsize, sync::Arc};
use tokio::sync::Mutex;
//内容缓存实现
pub struct MemoryCache {
    store: Arc<Mutex<LruCache<String, FetchResult>>>,
    ttl: Duration,
}

impl MemoryCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            store: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).expect(("capacity must be non-zero")),
            ))),
            ttl,
        }
    }

    pub async fn get(&self, key: &str) -> Result<FetchResult> {
        let mut store = self.store.lock().await;
        if let Some(result) = store
            .get(key)
            .filter(|res| chrono::Utc::now() - res.timestamp < self.ttl)
            .cloned()
        {
            Ok(result)
        } else {
            Err(anyhow!("Key not found or expired"))
        }
    }

    pub async fn store(&self, key: &str, result: FetchResult) {
        let mut store = self.store.lock().await;
        store.put(key.to_string(), result.clone());
    }
}
