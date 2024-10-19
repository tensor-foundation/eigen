use super::*;

use tensor_amm::{
    accounts::Pool,
    instructions::{CreatePool, CreatePoolInstructionArgs},
};

pub struct CreatePoolParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub whitelist: Pubkey,
}

pub fn create_pool(args: CreatePoolParams) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer = config.keypair.pubkey();
    let owner = config.keypair.pubkey();

    let create_pool_args: CreatePoolInstructionArgs =
        serde_json::from_reader(std::fs::File::open("pool_config.json")?)?;

    let pool = Pool::find_pda(&owner, create_pool_args.pool_id).0;

    let ix = CreatePool {
        rent_payer: payer,
        owner,
        pool,
        shared_escrow: None,
        whitelist: args.whitelist,
        system_program: solana_sdk::system_program::id(),
    }
    .instruction(create_pool_args);

    let tx = transaction!(&[&config.keypair], &[ix], &config.client);

    config.client.send_and_confirm_transaction(&tx)?;

    println!("Pool created: {}", pool);

    Ok(())
}
