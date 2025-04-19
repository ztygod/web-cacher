use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorConfig {
    pub task: Vec<MonitorTask>,
    pub global: GlobalConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorTask {
    pub url: String,

    #[serde(with = "humantime_serde")]
    pub interval: Duration,

    #[serde(with = "humantime_serde")]
    pub timeout: Duration,

    pub alert_rules: AlertRules,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlertRules {
    pub max_failures: u64,

    #[serde(with = "humantime_serde")]
    pub alert_interval: Duration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    #[serde(with = "humantime_serde")]
    pub cache_ttl: Duration,

    pub retry_policy: RetryPolicy,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryPolicy {
    pub max_retries: u32,

    #[serde(with = "humantime_serde")]
    pub backoff: Duration,
}

impl MonitorConfig {
    //查找默认配置文件
    pub fn config_path() -> Result<PathBuf> {
        Ok(PathBuf::from("config.toml"))
    }

    // 加载配置文件，异步读取toml，并反序列化
    pub async fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let content = tokio::fs::read_to_string(&path).await?;
        toml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse config from {}: {}", path.display(), e))
    }
}
