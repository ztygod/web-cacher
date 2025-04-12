use super::Cli;
use crate::Cli::Commands;

pub async fn handle_command(cli: Cli) -> Result<()> {
    match &cli.command {
        Command::Check(args) => handle_check(args).await,
        _ => {
            println!("该命令还未是实现");
            Ok(())
        }
    }
}

async fn handle_check(args: &CheckArgs) -> Result<()> {}
