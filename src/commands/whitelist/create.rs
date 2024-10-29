use super::*;

use tensor_whitelist::{
    instructions::{CreateWhitelistV2, CreateWhitelistV2InstructionArgs},
    types::Condition,
};
use uuid::Uuid;

pub struct CreateWhitelistV2Params {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub whitelist_config_path: PathBuf,
    pub namespace_path: Option<PathBuf>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct CreateWhitelistV2Config {
    uuid: Option<[u8; 32]>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    freeze_authority: Option<Pubkey>,
    conditions: Vec<Condition>,
}

pub fn create_whitelist_v2(args: CreateWhitelistV2Params) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer = config.keypair.pubkey();
    let owner = config.keypair.pubkey();

    let namespace_signer = if let Some(namespace_path) = args.namespace_path {
        read_keypair_file(namespace_path).map_err(|_| anyhow!("Unable to read keypair file"))?
    } else {
        Keypair::new()
    };

    let create_whitelist_config: CreateWhitelistV2Config =
        serde_json::from_reader(std::fs::File::open(args.whitelist_config_path)?)?;

    let uuid = create_whitelist_config.uuid.unwrap_or_else(|| {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let mut extended_uuid = [0u8; 32];
        extended_uuid[..16].copy_from_slice(uuid1.as_bytes());
        extended_uuid[16..].copy_from_slice(uuid2.as_bytes());
        extended_uuid
    });

    let whitelist = WhitelistV2::find_pda(&namespace_signer.pubkey(), uuid).0;

    let args = CreateWhitelistV2InstructionArgs {
        uuid,
        freeze_authority: create_whitelist_config.freeze_authority,
        conditions: create_whitelist_config.conditions,
    };

    let ix = CreateWhitelistV2 {
        payer,
        update_authority: owner,
        namespace: namespace_signer.pubkey(),
        whitelist,
        system_program: solana_sdk::system_program::id(),
    }
    .instruction(args);

    let tx = transaction!(&[&config.keypair, &namespace_signer], &[ix], &config.client);

    config.client.send_and_confirm_transaction(&tx)?;

    println!("Whitelist created: {}", whitelist);
    println!("Namespace: {}", namespace_signer.pubkey());

    Ok(())
}
