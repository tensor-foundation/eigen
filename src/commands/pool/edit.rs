use super::*;

use tensor_amm::instructions::{EditPool, EditPoolInstructionArgs};

pub struct EditPoolParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub pool: Pubkey,
    pub edit_pool_config_path: PathBuf,
}

pub fn edit_pool(args: EditPoolParams) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let owner = config.keypair.pubkey();

    let edit_pool_instruction_args: EditPoolInstructionArgs =
        serde_json::from_reader(std::fs::File::open(args.edit_pool_config_path)?)?;

    let ix = EditPool {
        owner,
        pool: args.pool,
        system_program: solana_sdk::system_program::id(),
    }
    .instruction(edit_pool_instruction_args);

    let tx = transaction!(&[&config.keypair], &[ix], &config.client);

    config.client.send_and_confirm_transaction(&tx)?;

    println!("Pool updated: {}", args.pool);

    Ok(())
}
