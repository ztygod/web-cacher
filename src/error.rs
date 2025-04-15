use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid config value: {0}")]
    InvalidValue(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),
}
