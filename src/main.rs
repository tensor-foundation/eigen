use anyhow::Result;
use clap::Parser;

use eigen::{
    args::{Args, Commands, FeesSubcommands, WhitelistSubcommands},
    commands::{
        fund_shards, generate_fee_shards, get_shard_balances, handle_compare, handle_decode,
        handle_download, CompareArgs, DecodeArgs, DownloadArgs, FeeArgs,
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
        Commands::Fees(subcommand) => match subcommand {
            FeesSubcommands::Shards => generate_fee_shards(),
            FeesSubcommands::Fund => fund_shards(FeeArgs {
                keypair_path,
                rpc_url,
            }),
            FeesSubcommands::Balances => get_shard_balances(FeeArgs {
                keypair_path,
                rpc_url,
            }),
        },
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
