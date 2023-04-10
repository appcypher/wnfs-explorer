pub mod config;
pub mod diff;
pub mod merge;
pub mod open;
pub mod repl;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use wnfs::libipld::Cid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Verb>,
}

#[derive(Subcommand)]
pub enum Verb {
    Config(Config),
    Repl {},
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
pub enum Noun {
    Fs,
    Hamt,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(about = "Configure the CLI")]
pub struct Config {
    #[arg(short = 'u', long = "store-url")]
    pub store_url: String,
    #[arg(short = 'r', long = "root-tree")]
    pub root_tree_cid: String,
}
