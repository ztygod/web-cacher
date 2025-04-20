pub mod client;
pub mod diff;
pub mod health;

use crate::error::Error;
use anyhow::{anyhow, Result};
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

    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,

    pub content: Vec<u8>,
    pub hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn check_url(client: &CustomClient, url: &str, timeout: Duration) -> Result<CheckResult> {
    if !url.starts_with("http") {
        return Err(anyhow!("check_url failure"));
    }

    // 记录开始时间用于计算总耗时
    let start_time = std::time::Instant::now();

    //执行实际请求
    let fetch_result = client.fetch(url, timeout).await?;

    //计算响应时间
    let duration = start_time.elapsed();

    // 构建检查结果
    Ok(CheckResult {
        url: url.to_string(),
        status: fetch_result.status,
        content: String::from_utf8_lossy(&fetch_result.content).into_owned(),
        hash: fetch_result.hash,
        duration,
        timestamp: fetch_result.timestamp,
    })
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub url: String,
    pub status: reqwest::StatusCode,
    pub content: String,
    pub hash: String,
    pub duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl CheckResult {
    pub fn is_health(&self) -> bool {
        self.status.is_success()
    }

    //获取内容预览
    pub fn preview(&self) -> &str {
        &self.content[..self.content.len().min(200)]
    }
}
