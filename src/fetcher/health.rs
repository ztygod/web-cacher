use anyhow::{Context, Ok, Result};
use reqwest::{Client, StatusCode};

pub struct HealthStatus {
    pub status: StatusCode,
    pub content: String,
    pub is_success: bool,
}

impl HealthStatus {
    pub fn is_health(&self) -> bool {
        self.status.is_success() || self.status.is_redirection()
    }
}

pub async fn check_out(client: &Client, url: &str) -> Result<HealthStatus> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send request")?;

    let status = response.status();
    let content = response
        .text()
        .await
        .context("Failed to read response body")?;

    Ok(HealthStatus {
        status,
        content,
        is_success: status.is_success(),
    })
}
