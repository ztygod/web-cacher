mod cache;
mod cli;
mod config;
mod daemon;
mod error;
mod fetcher;
mod scheduler;
mod utils;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::args;
use cli::{Cli, Commands};
use tracing_subscriber;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // 1.解析命令行参数
    let cli = Cli::parse();

    // 2.设置日志等级
    println!("📝 日志等级: {}", cli.log_level);
    println!("📄 配置文件: {:?}", cli.config);

    // 3.分发命令
    match &cli.command {
        Commands::Run(args) => {
            println!("🚀 启动服务中...");
            println!("🛡️ 守护模式: {}", args.daemon);
            println!("📊 指标地址: {}", args.metrics_addr);
            println!("⏱️ 立即执行: {}", args.immediate);
        }

        Commands::Check(args) => {
            println!("🔍正在检查URL: {:?}", args.urls);
            if args.json {
                println!("📦 输出格式: JSON");
            }
            if args.verbose {
                println!("🔧 显示详细响应")
            }
        }

        Commands::Cache(cache_cmd) => match &cache_cmd {
            args::CacheCommands::Clean(args) => {
                println!("🧹 清理缓存（保留 {} 个）", args.keep);
                if let Some(filter) = &args.filter {
                    println!("🔍 URL过滤: {}", filter);
                }
                println!("⚠️ 强制执行: {}", args.force);
            }
            args::CacheCommands::List(args) => {
                println!("📋 列出缓存");
                if let Some(filter) = &args.filter {
                    println!("🔍 URL过滤: {}", filter);
                }
                println!("📎 显示详细信息: {}", args.detail);
            }
        },

        Commands::Report(args) => {
            println!("📊 生成报告");
            println!("📁 输出到文件: {}", args.output);
            println!("🕒 范围: {:?}", args.range);
            println!("🧾 格式: {}", args.format);
        }

        Commands::Init => {
            println!("🛠️ 初始化配置文件...");
            // todo: 初始化配置逻辑
        }

        Commands::Completions(args) => {
            use clap_complete::generate_to;
            //use std::io;

            println!("🔧 生成 {:?} 补全脚本到 {:?}", args.shell, args.output);
            let mut cmd = Cli::command();
            let shell = args.shell;
            let output_dir = &args.output;

            match generate_to(shell, &mut cmd, "web_cache", output_dir) {
                Ok(path) => println!("✅ 补全脚本生成成功: {:?}", path),
                Err(e) => eprintln!("❌ 生成失败: {}", e),
            }
        }
    }

    // 4.命令行执行
    cli::handler::handle_command(cli).await
}
