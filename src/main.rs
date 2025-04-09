mod cli;

use clap::Parser;
use cli::Cli;
fn main() {
    let arg = Cli::parse();
}
