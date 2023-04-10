use anyhow::Result;
use clap::Parser;
use wnfs_cli::{config, Verb, Cli, repl, open, diff, merge};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Verb::Config(config)) => {
            config::handle(config)?;
        }
        Some(Verb::Repl {}) => {
            repl::handle()?;
        }
        Some(Verb::Diff { noun }) => {
            diff::handle(noun)?;
        }
        Some(Verb::Open { noun }) => {
            open::handle(noun).await?;
        }
        Some(Verb::Merge { noun }) => {
            merge::handle(noun)?;
        }
        None => {
            Cli::parse_from(["wnfs-cli", "--help"]);
        }
    }

    Ok(())
}
