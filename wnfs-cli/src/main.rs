mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{diff, merge, open};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Open {},
    Diff {},
    Merge {},
}

/// Main entry point.
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Open {}) => {
            open::test_open()?;
        }
        Some(Commands::Diff {}) => {
            diff::test_diff();
        }
        Some(Commands::Merge {}) => {
            merge::test_merge();
        }
        None => {}
    }

    Ok(())
}
