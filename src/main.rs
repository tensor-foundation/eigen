use anyhow::Result;
use clap::Parser;

use tensor_eigen::{
    args::{
        Args, Commands, DeriveSubcommands, EigenSubcommands, FeesSubcommands, PoolSubcommands,
        WhitelistSubcommands,
    },
    commands::{
        create_pool, create_whitelist_v2, edit_pool, fund_shards, generate_fee_shards,
        get_shard_balances, handle_anchor_discriminator, handle_compare, handle_decode,
        handle_download, handle_error, update_eigen, update_whitelist_v2, CompareParams,
        CreatePoolParams, CreateWhitelistV2Params, DecodeParams, DownloadParams, EditPoolParams,
        ErrorParams, FeeParams, UpdateWhitelistV2Params,
    },
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    match args.command {
        Commands::Decode(args) => handle_decode(DecodeParams {
            rpc_url: args.read_options.rpc_url,
            address: args.address,
            raw: args.raw,
        }),
        Commands::Derive(subcommand) => match subcommand {
            DeriveSubcommands::AnchorDisc(args) => handle_anchor_discriminator(args),
        },
        Commands::Download(args) => handle_download(DownloadParams {
            rpc_url: args.read_options.rpc_url,
            address: args.address,
            output_dir: args.output_dir,
        }),
        Commands::Error(args) => handle_error(ErrorParams {
            error_code: args.error_code,
        }),
        Commands::Eigen(subcommand) => match subcommand {
            EigenSubcommands::Update => update_eigen(),
        },
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
                pool_config_path: args.pool_config_path,
            }),
            PoolSubcommands::Edit(args) => edit_pool(EditPoolParams {
                keypair_path: args.write_options.keypair_path,
                rpc_url: args.write_options.rpc_url,
                pool: args.pool,
                edit_pool_config_path: args.edit_pool_config_path,
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
                whitelist_config_path: args.whitelist_config_path,
            }),
            WhitelistSubcommands::Update(args) => update_whitelist_v2(UpdateWhitelistV2Params {
                keypair_path: args.write_options.keypair_path,
                rpc_url: args.write_options.rpc_url,
                whitelist_address: args.whitelist_address,
                new_conditions_path: args.new_conditions_path,
                new_update_authority_path: args.new_update_authority_path,
                new_freeze_authority: args.new_freeze_authority,
            }),
        },
    }
}
