use anyhow::Result;
use clap::Parser;

use eigen::{
    args::{Args, Commands, WhitelistSubcommands},
    commands::{
        handle_compare, handle_decode, handle_download, CompareArgs, DecodeArgs, DownloadArgs,
    },
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Decode { address } => handle_decode(DecodeArgs {
            keypair_path,
            rpc_url,
            address,
        }),
        Commands::Download {
            address,
            output_dir,
        } => handle_download(DownloadArgs {
            keypair_path,
            rpc_url,
            address,
            output_dir,
        }),
        Commands::Whitelist(subcommand) => match subcommand {
            WhitelistSubcommands::Compare { list, verbose } => handle_compare(CompareArgs {
                keypair_path,
                rpc_url,
                list,
                verbose,
            }),
        },
    }
}
