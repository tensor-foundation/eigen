use anyhow::{anyhow, Result};
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tensor_amm::accounts::{NftDepositReceipt, Pool};
use tensor_marketplace::accounts::{BidState, ListState};
use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

use crate::{commands::Discriminator, discriminators::deserialize_account, setup::CliConfig};

pub struct DownloadParams {
    pub rpc_url: Option<String>,
    pub address: Pubkey,
    pub output_dir: Option<PathBuf>,
}

pub fn handle_download(args: DownloadParams) -> Result<()> {
    let config = CliConfig::new(None, args.rpc_url)?;

    let data = config.client.get_account_data(&args.address)?;
    let discriminator = &data[0..8];

    let (account_data, file_name) = match discriminator {
        d if d == Pool::discriminator() => {
            let pool = deserialize_account::<Pool>(&data)?;
            (serde_json::to_value(&pool)?, "pool.json")
        }
        d if d == NftDepositReceipt::discriminator() => {
            let nft_deposit_receipt = deserialize_account::<NftDepositReceipt>(&data)?;
            (
                serde_json::to_value(&nft_deposit_receipt)?,
                "nft_deposit_receipt.json",
            )
        }
        d if d == Whitelist::discriminator() => {
            let whitelist = deserialize_account::<Whitelist>(&data)?;
            (serde_json::to_value(&whitelist)?, "whitelist.json")
        }
        d if d == WhitelistV2::discriminator() => {
            let whitelist_v2 = deserialize_account::<WhitelistV2>(&data)?;
            (serde_json::to_value(&whitelist_v2)?, "whitelist_v2.json")
        }
        d if d == MintProof::discriminator() => {
            let mint_proof = deserialize_account::<MintProof>(&data)?;
            (serde_json::to_value(&mint_proof)?, "mint_proof.json")
        }
        d if d == MintProofV2::discriminator() => {
            let mint_proof_v2 = deserialize_account::<MintProofV2>(&data)?;
            (serde_json::to_value(&mint_proof_v2)?, "mint_proof_v2.json")
        }
        d if d == BidState::discriminator() => {
            let bid_state = deserialize_account::<BidState>(&data)?;
            (serde_json::to_value(&bid_state)?, "bid_state.json")
        }
        d if d == ListState::discriminator() => {
            let list_state = deserialize_account::<ListState>(&data)?;
            (serde_json::to_value(&list_state)?, "list_state.json")
        }
        _ => return Err(anyhow!("Unsupported discriminator")),
    };

    let output_path = args.output_dir.unwrap_or_else(|| PathBuf::from("."));
    let mut file = File::create(output_path.join(file_name))?;
    let json_string = serde_json::to_string_pretty(&account_data)?;
    file.write_all(json_string.as_bytes())?;

    println!("Account data saved to {}", output_path.display());

    Ok(())
}
