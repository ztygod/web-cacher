use clap::Parser;
#[derive(Parser, Debug)]
#[command(name = "web-cacher")]
#[command(about = "异步网页监控与缓存工具",long_about=None)]

pub struct Cli {
    /// 指定路径
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    /// 是否启用调试模式
    #[arg(short, long)]
    pub debug: bool,
}
