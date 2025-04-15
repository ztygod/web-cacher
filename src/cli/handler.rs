use std::time::{Duration, Instant};

use anyhow::Result;
use futures::future::join_all;
use humantime::format_duration;
use reqwest::Client;
use serde_json::json;
use tracing::info;

use super::{
    args::{CheckArgs, Commands, RunArgs},
    Cli,
};
use crate::{
    daemon::daemonize,
    fetcher::{fetch_url, health::check_out},
    utils::sha256,
};

pub async fn handle_command(cli: Cli) -> Result<()> {
    match &cli.command {
        Commands::Check(args) => handle_check(args).await,
        Commands::Run(args) => handle_run(args).await,
        _ => {
            println!("该命令还未是实现");
            Ok(())
        }
    }
}

async fn handle_check(args: &CheckArgs) -> Result<()> {
    info!("Starting health check for {} URLs", args.urls.len());

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .build()?;

    let tasks = args.urls.iter().map(|url| {
        let url = url.clone();
        let client = client.clone();
        async move {
            let start_time = Instant::now();
            let result = check_out(&client, &url).await;
            let duration = start_time.elapsed();
            (url, result, duration)
        }
    });

    let results = join_all(tasks).await;
    let mut output = Vec::new();

    for (url, result, duration) in results {
        let mut entry = json!({
            "url":url,
            "duration_ms": duration.as_millis(),
            "duration_human": format_duration(duration).to_string(),
        });

        match result {
            Ok(status) => {
                let success = status.is_health();
                let hash = sha256(&status.content);

                let mut status_json = json!({
                    "healthy":success,
                    "status_code":status.status.as_u16(),
                    "content_length":status.content.len(),
                    "content_hash":hash,
                });

                if args.verbose {
                    status_json["preview"] =
                        json!(status.content.chars().take(200).collect::<String>());
                }

                entry["status"] = status_json;

                if !args.json {
                    println!(
                        "{} [{}] {} - {}ms\n Hash: {}",
                        if success { "✅" } else { "⚠️ " },
                        status.status,
                        url,
                        duration.as_millis(),
                        hash
                    );
                    if args.verbose {
                        println!(
                            " Preview: {}",
                            &status.content[..status.content.len().min(200)]
                        );
                    }
                }
            }
            Err(e) => {
                entry["error"] = json!(e.to_string());

                if !args.json {
                    eprintln!("❌ [{}] Error: {}", url, e);
                }
            }
        }
        output.push(entry);
    }
    if args.json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    }
    Ok(())
}

async fn handle_run(args: &RunArgs) -> Result<()> {
    if args.daemon {
        daemonize()?;
    }

    info!("Starting web-cacher service");

    // 加载配置
}
