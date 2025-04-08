mod cli;

use clap::Parser;
use cli::Cli;
fn main() {
    let arg = Cli::parse();

    println!("配置文件路径: {}", arg.config);
    println!("是否调试模式: {}", arg.debug);
}
