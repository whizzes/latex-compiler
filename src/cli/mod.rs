mod command;

use anyhow::Result;
use clap::Parser;

use command::serve::ServeCmd;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        self.command.exec().await
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "latex-compiler",
    about = "Latex Compiler Command Line Interface",
    author = "Whizzes Developers",
    max_term_width = 100,
    next_line_help = true
)]
pub enum Command {
    /// Serves the Latex Compiler as a web service
    Serve(ServeCmd),
}

impl Command {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Serve(cmd) => cmd.exec().await,
        }
    }
}
