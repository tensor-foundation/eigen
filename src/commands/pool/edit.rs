use super::*;

use borsh::BorshDeserialize;
use serde::{Deserialize, Serialize};
use tensor_amm::{
    instructions::{EditPool, EditPoolInstructionArgs},
    types::{CurveType, PoolConfig, PoolType},
    NullableU16,
};

pub struct EditPoolParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub pool: Pubkey,
    pub edit_pool_config_path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
struct EditPoolArgs {
    pub new_config: Option<EditPoolConfig>,
    pub cosigner: Option<Pubkey>,
    pub expire_in_sec: Option<u64>,
    pub max_taker_sell_count: Option<u32>,
    pub reset_price_offset: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EditPoolConfig {
    pub curve_type: CurveType,
    pub starting_price: u64,
    pub delta: u64,
    pub mm_compound_fees: bool,
    pub mm_fee_bps: NullableU16,
}

impl EditPoolConfig {
    fn convert(&self, pool_type: PoolType) -> PoolConfig {
        PoolConfig {
            pool_type,
            curve_type: self.curve_type,
            starting_price: self.starting_price,
            delta: self.delta,
            mm_compound_fees: self.mm_compound_fees,
            mm_fee_bps: self.mm_fee_bps.clone(),
        }
    }
}

impl EditPoolArgs {
    fn convert(&self, pool_type: PoolType) -> EditPoolInstructionArgs {
        EditPoolInstructionArgs {
            new_config: self.new_config.as_ref().map(|c| c.convert(pool_type)),
            cosigner: self.cosigner,
            expire_in_sec: self.expire_in_sec,
            max_taker_sell_count: self.max_taker_sell_count,
            reset_price_offset: self.reset_price_offset,
        }
    }
}

pub fn edit_pool(args: EditPoolParams) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let owner = config.keypair.pubkey();

    // Fetch and decode pool account.
    let pool_data = config.client.get_account_data(&args.pool)?;
    let pool_type = Pool::try_from_slice(&pool_data)?.config.pool_type;

    let edit_pool_args: EditPoolArgs =
        serde_json::from_reader(std::fs::File::open(args.edit_pool_config_path)?)?;

    println!("{:?}", edit_pool_args);

    let ix = EditPool {
        owner,
        pool: args.pool,
        system_program: solana_sdk::system_program::id(),
    }
    .instruction(edit_pool_args.convert(pool_type));

    let tx = transaction!(&[&config.keypair], &[ix], &config.client);

    config.client.send_and_confirm_transaction(&tx)?;

    println!("Pool updated: {}", args.pool);

    Ok(())
}
