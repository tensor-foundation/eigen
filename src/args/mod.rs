use std::path::PathBuf;

mod derive;
mod eigen;
mod fees;
mod pool;
mod whitelist;

pub use derive::*;
pub use eigen::*;
pub use fees::*;
pub use pool::*;
pub use whitelist::*;

use clap::{Args as ClapArgs, Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

use crate::commands::Id;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Decode(DecodeArgs),

    #[clap(subcommand)]
    Derive(DeriveSubcommands),

    Download(DownloadArgs),

    Error(ErrorArgs),

    #[clap(subcommand)]
    Fees(FeesSubcommands),

    Ids(IdArgs),

    #[clap(subcommand)]
    Pool(PoolSubcommands),

    #[clap(subcommand, name = "self")]
    Eigen(EigenSubcommands),

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

    /// Print raw bytes.
    #[arg(long)]
    pub raw: bool,
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

#[derive(ClapArgs)]
pub struct IdArgs {
    /// ID name.
    pub id: Id,
}
