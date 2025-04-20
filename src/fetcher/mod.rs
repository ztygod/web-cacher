pub mod client;
pub mod diff;
pub mod health;

use anyhow::Result;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Duration;
#[derive(Clone)]
pub struct CustomClient {
    inner: Client,
}

impl CustomClient {
    pub fn new() -> Self {
        Self {
            inner: Client::builder()
                .user_agent("Mozilla/5.0 (compatible; WebMonitor/1.0)")
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    //执行请求并计算内容哈希
    pub async fn fetch(&self, url: &str, timeout: Duration) -> Result<FetchResult> {
        let response = self.inner.get(url).timeout(timeout).send().await?;

        let status = response.status();
        let content = self.get_response_content(response).await?;
        let hash = self.calculate_hash(&content);

        Ok(FetchResult {
            url: url.to_string(),
            status,
            content,
            hash,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn get_response_content(&self, mut response: Response) -> Result<Vec<u8>> {
        use bytes::Bytes;
        use std::io::Cursor;

        let mut content = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            let chunk: Bytes = chunk;
            let mut cursor = Cursor::new(chunk);
            std::io::copy(&mut cursor, &mut content)?;
        }
        Ok(content)
    }

    fn calculate_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FetchResult {
    pub url: String,
    pub status: StatusCode,
    pub content: Vec<u8>,
    pub hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
