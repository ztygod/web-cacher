use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::error::ConfigError;

const DEFAULT_CONFIG: &str = include_str!("../default_config.toml");
#[derive(Debug, Deserialize)]
pub struct Config {
    pub check_interval: u64,
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub url: String,
    pub css_selector: Option<String>,
}

pub fn load() -> Result<Config> {
    // 优先尝试用户配置文件
    let config_path = Path::new("config.toml");

    let config_str = if config_path.exists() {
        fs::read_to_string(config_path).context("Failed to read config file")?;
    } else {
        DEFAULT_CONFIG.to_string()
    };

    //解析TOML
    let config: Config = toml::from_str(&config_str).context("Invalid config format")?;

    //验证配置
    vaildate(&config)?;
    Ok(config)
}

fn vaildate(config: &Config) -> Result<()> {
    if config.check_interval == 0 {
        return Err(
            ConfigError::InvalidValue("check_interval must be greater than 0".into()).into(),
        );
    }

    for target in &config.targets {
        if target.url.is_empty() {
            return Err(ConfigError::InvalidValue("target.url cannot be empty".into()).into());
        }
    }
    Ok(())
}
