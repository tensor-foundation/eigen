use std::path::PathBuf;

mod fees;
mod pool;
mod whitelist;

pub use fees::*;
pub use pool::*;
pub use whitelist::*;

use clap::{Args as ClapArgs, Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Decode(DecodeArgs),

    Download(DownloadArgs),

    Error(ErrorArgs),

    #[clap(subcommand)]
    Fees(FeesSubcommands),

    #[clap(subcommand)]
    Pool(PoolSubcommands),

    #[clap(subcommand)]
    Whitelist(WhitelistSubcommands),
}

// Global options for read commands
#[derive(ClapArgs)]
pub struct ReadOptions {
    /// RPC URL for the Solana cluster.
    #[arg(short, long)]
    pub rpc_url: Option<String>,
}

// Global options for write commands
#[derive(ClapArgs)]
pub struct WriteOptions {
    /// Path to the keypair file.
    #[arg(short, long)]
    pub keypair_path: Option<PathBuf>,

    /// RPC URL for the Solana cluster.
    #[arg(short, long)]
    pub rpc_url: Option<String>,
}

#[derive(ClapArgs)]
pub struct DecodeArgs {
    #[command(flatten)]
    pub read_options: ReadOptions,

    /// Address to decode.
    pub address: Pubkey,
}

#[derive(ClapArgs)]
pub struct DownloadArgs {
    #[clap(flatten)]
    pub read_options: ReadOptions,

    /// Address to download.
    pub address: Pubkey,

    /// Output directory.
    pub output_dir: Option<PathBuf>,
}

#[derive(ClapArgs)]
pub struct ErrorArgs {
    /// Error code.
    pub error_code: String,
}
