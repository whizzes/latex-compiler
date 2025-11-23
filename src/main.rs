mod cli;
mod latex;
mod router;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec().await?;
    Ok(())
}
