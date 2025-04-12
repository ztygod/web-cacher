pub mod args;
pub mod handler;

use clap::Parser;
use std::path::PathBuf;

pub use args::Commands;

/// 主CLI结构
#[derive(Parser, Debug)]
#[clap(
    name = "web-cacher",
    version,
    about = "🗂️ Web监控缓存工具",
    long_about = "高性能网页监控与内容存档工具，支持智能缓存、变化检测和可视化报告",
    arg_required_else_help = true
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    ///全局日志级别
    #[clap(global = true, short, long, default_value = "info")]
    pub log_level: String,

    ///指定配置文件路径
    #[clap(global = true, short, long, default_value = "config.toml")]
    pub config: PathBuf,
}
