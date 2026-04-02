use anyhow::Result;
use clap::Parser;
use pgsql_query::{cli::Cli, commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli).await
}
