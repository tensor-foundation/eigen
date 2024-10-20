use super::*;

use solana_sdk::signature::read_keypair_file;
use tensor_whitelist::{
    instructions::{UpdateWhitelistV2, UpdateWhitelistV2InstructionArgs},
    types::{Condition, Operation},
};

pub struct UpdateWhitelistV2Params {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub whitelist_address: Pubkey,
    pub new_conditions_path: Option<PathBuf>,
    pub new_update_authority_path: Option<PathBuf>,
    pub new_freeze_authority: Option<Pubkey>,
}

pub fn update_whitelist_v2(args: UpdateWhitelistV2Params) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer = config.keypair.pubkey();
    let owner = config.keypair.pubkey();

    let new_conditions: Option<Vec<Condition>> = args.new_conditions_path.map(|path| {
        serde_json::from_reader(
            std::fs::File::open(path).expect("Failed to open new conditions file"),
        )
        .expect("Failed to parse new conditions")
    });

    let freeze_authority = match args.new_freeze_authority {
        Some(pubkey) => Operation::Set(pubkey),
        None => Operation::Noop,
    };

    let update_args = UpdateWhitelistV2InstructionArgs {
        freeze_authority,
        conditions: new_conditions,
    };

    let new_update_authority = args
        .new_update_authority_path
        .map(|path| read_keypair_file(path).expect("Failed to read new update authority keypair"));

    let ix = UpdateWhitelistV2 {
        payer,
        update_authority: owner,
        whitelist: args.whitelist_address,
        new_update_authority: new_update_authority.as_ref().map(|k| k.pubkey()),
        system_program: solana_sdk::system_program::id(),
    }
    .instruction(update_args);

    let signers = if let Some(ref new_auth) = new_update_authority {
        vec![&config.keypair, new_auth]
    } else {
        vec![&config.keypair]
    };
    let tx = transaction!(&signers, &[ix], &config.client);

    config.client.send_and_confirm_transaction(&tx)?;

    println!("Whitelist updated: {}", args.whitelist_address);

    Ok(())
}
