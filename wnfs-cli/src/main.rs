mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{diff, merge, open};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Verb>,
}

#[derive(Subcommand)]
enum Verb {
    Open {
        #[clap(subcommand)]
        noun: Noun,
    },
    Diff {
        #[clap(subcommand)]
        noun: Noun,
    },
    Merge {
        #[clap(subcommand)]
        noun: Noun,
    },
}

#[derive(Subcommand)]
enum Noun {
    Fs,
    Hamt,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Verb::Open { noun }) => {
            open::handle(noun)?;
        }
        Some(Verb::Diff { noun }) => {
            diff::handle(noun);
        }
        Some(Verb::Merge { noun }) => {
            merge::handle(noun);
        }
        None => {
            Cli::parse_from(&["wnfs-cli", "--help"]);
        }
    }

    Ok(())
}
