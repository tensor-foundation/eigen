use anyhow::Result;
use clap::Parser;

use eigen::{
    args::{Args, Commands, FeesSubcommands, PoolSubcommands, WhitelistSubcommands},
    commands::{
        create_pool, create_whitelist_v2, fund_shards, generate_fee_shards, get_shard_balances,
        handle_compare, handle_decode, handle_download, handle_error, CompareArgs, CreatePoolArgs,
        CreateWhitelistV2Args, DecodeArgs, DownloadArgs, ErrorArgs, FeeArgs,
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
        Commands::Error { error_code } => handle_error(ErrorArgs { error_code }),
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
        Commands::Pool(subcommand) => match subcommand {
            PoolSubcommands::Create { whitelist } => create_pool(CreatePoolArgs {
                keypair_path,
                rpc_url,
                whitelist,
            }),
        },
        Commands::Whitelist(subcommand) => match subcommand {
            WhitelistSubcommands::Compare { list, verbose } => handle_compare(CompareArgs {
                keypair_path,
                rpc_url,
                list,
                verbose,
            }),
            WhitelistSubcommands::Create { namespace_path } => {
                create_whitelist_v2(CreateWhitelistV2Args {
                    keypair_path,
                    rpc_url,
                    namespace_path,
                })
            }
        },
    }
}
