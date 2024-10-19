use anyhow::Result;
use clap::Parser;

use tensor_eigen::{
    args::{Args, Commands, FeesSubcommands, PoolSubcommands, WhitelistSubcommands},
    commands::{
        create_pool, create_whitelist_v2, fund_shards, generate_fee_shards, get_shard_balances,
        handle_compare, handle_decode, handle_download, handle_error, CompareParams,
        CreatePoolParams, CreateWhitelistV2Params, DecodeParams, DownloadParams, ErrorParams,
        FeeParams,
    },
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    match args.command {
        Commands::Decode(args) => handle_decode(DecodeParams {
            rpc_url: args.read_options.rpc_url,
            address: args.address,
        }),
        Commands::Download(args) => handle_download(DownloadParams {
            rpc_url: args.read_options.rpc_url,
            address: args.address,
            output_dir: args.output_dir,
        }),
        Commands::Error(args) => handle_error(ErrorParams {
            error_code: args.error_code,
        }),
        Commands::Fees(subcommand) => match subcommand {
            FeesSubcommands::Shards => generate_fee_shards(),
            FeesSubcommands::Fund(args) => fund_shards(FeeParams {
                keypair_path: args.write_options.keypair_path,
                rpc_url: args.write_options.rpc_url,
            }),
            FeesSubcommands::Balances(args) => get_shard_balances(FeeParams {
                keypair_path: None,
                rpc_url: args.read_options.rpc_url,
            }),
        },
        Commands::Pool(subcommand) => match subcommand {
            PoolSubcommands::Create(args) => create_pool(CreatePoolParams {
                keypair_path: args.write_options.keypair_path,
                rpc_url: args.write_options.rpc_url,
                whitelist: args.whitelist,
            }),
        },
        Commands::Whitelist(subcommand) => match subcommand {
            WhitelistSubcommands::Compare(args) => handle_compare(CompareParams {
                keypair_path: None,
                rpc_url: args.read_options.rpc_url,
                list: args.list,
                verbose: args.verbose,
            }),
            WhitelistSubcommands::Create(args) => create_whitelist_v2(CreateWhitelistV2Params {
                keypair_path: args.write_options.keypair_path,
                rpc_url: args.write_options.rpc_url,
                namespace_path: args.namespace_path,
            }),
        },
    }
}
