use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the keypair file.
    #[arg(short, long, global = true)]
    pub keypair_path: Option<PathBuf>,

    /// RPC URL for the Solana cluster.
    #[arg(short, long, global = true)]
    pub rpc_url: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Decode {
        address: Pubkey,
    },
    Download {
        address: Pubkey,
        output_dir: Option<PathBuf>,
    },
    #[clap(subcommand)]
    Fees(FeesSubcommands),
    #[clap(subcommand)]
    Whitelist(WhitelistSubcommands),
}

#[derive(Clone, Subcommand)]
pub enum FeesSubcommands {
    Shards,
    Fund,
    Balances,
}

#[derive(Clone, Subcommand)]
pub enum WhitelistSubcommands {
    Compare {
        list: Option<PathBuf>,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}
