use clap::{Args, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;
use std::time::Duration;

///主命令枚举
#[derive(Debug, Subcommand)]
pub enum Commands {
    ///启功服务监控
    Run(RunArgs),

    ///手动检查单个URL
    #[clap(visible_alias = "chk")]
    Check(CheckArgs),

    ///缓存管理
    #[clap(subcommand)]
    Cache(CacheCommands),

    /// 生成监控报告
    Report(ReportArgs),

    /// 初始化配置文件
    Init,

    /// 生成Shell自动补全
    Completions(CompletionsArgs),
}

///运行服务参数
#[derive(Debug, Args)]
pub struct RunArgs {
    ///后台守护进程模式
    #[clap(short, long)]
    pub daemon: bool,

    ///监控指标暴露地址
    #[clap(long, default_value = "127.0.0.1:9090")]
    pub metrics_addr: String,

    ///强制立即运行首次检查
    #[clap(short, long)]
    pub immediate: bool,
}

///手动检查参数
#[derive(Debug, Args)]
pub struct CheckArgs {
    ///要检查的url列表
    #[clap(required = true)]
    pub urls: Vec<String>,

    /// 输出JSON格式结果
    #[clap(short, long)]
    pub json: bool,

    /// 显示完整响应内容
    #[clap(short, long)]
    pub verbose: bool,

    ///超时时间(秒)
    #[clap(short, long, default_value = "10")]
    pub timeout: u64,
}

/// 缓存子命令
#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    ///清理旧缓存版本
    Clean(CacheCleanArgs),

    ///列出缓存内容
    #[clap(visible_alias = "ls")]
    List(CacheListArgs),
}

///缓存清理参数
#[derive(Debug, Args)]
pub struct CacheCleanArgs {
    ///保留的版本数量
    #[clap(short, long, default_value = "5")]
    pub keep: usize,

    ///按URL模式过滤
    #[clap(short, long)]
    pub filter: Option<String>,

    ///不询问直接执行
    #[clap(short, long)]
    pub force: bool,
}

///缓存列表参数
#[derive(Debug, Args)]
pub struct CacheListArgs {
    /// 按URL模式过滤
    #[clap(short, long)]
    pub filter: Option<String>,

    /// 显示详细缓存信息
    #[clap(short, long)]
    pub detail: bool,
}

/// 报告生成参数
#[derive(Debug, Args)]
pub struct ReportArgs {
    ///输出文件路径
    #[clap(short, long)]
    pub output: bool,

    ///报告时间范围
    #[clap(short,long,value_parser = parse_duration)]
    pub range: Option<humantime::Duration>,

    /// 生成报告格式
    #[clap(short, long, default_value = "html")]
    pub format: String,
}

/// Shell自动补全参数
#[derive(Debug, Args)]
pub struct CompletionsArgs {
    /// 要生成的Shell类型
    #[clap(value_enum)]
    pub shell: Shell,

    /// 输出目录
    #[clap(short, long, default_value = ".")]
    pub output: PathBuf,
}

///自定义时长解析器
fn parse_duration(s: &str) -> Result<Duration, humantime::DurationError> {
    Ok(humantime::parse_duration(s)?)
}
