use humantime::Duration;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorConfig {
    pub tasks: Vec<MonitorTask>,
    pub global:GlobalConfig,
}

pub struct MonitorTask {
    pub url: String,
    pub interval: Duration,
    pub timeout: Duration,
    //pub alert_rules:
}

pub struct GlobalConfig{
    pub cache_ttl:Duration,
    pub retry_policy:
}

impl MonitorConfig {
    pub async fn load() ->Result<Self>{
        let path = Self::c
    }
}
