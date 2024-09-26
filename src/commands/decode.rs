use anyhow::{anyhow, Result};
use solana_sdk::{account::Account, pubkey::Pubkey};
use tensor_amm::accounts::{NftDepositReceipt, Pool};
use tensor_marketplace::accounts::{BidState, ListState};
use tensor_price_lock::accounts::OrderState;
use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

use crate::{discriminators::deserialize_account, formatting::CustomFormat};

use super::*;

pub struct DecodeArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub address: Pubkey,
}

pub fn handle_decode(args: DecodeArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let account = config.client.get_account(&args.address)?;

    if is_wallet_type(&account) {
        println!("{}", account.custom_format());
        return Ok(());
    }

    let data = account.data.as_slice();

    if data.len() < 8 {
        return Err(anyhow!("No account discriminator found!"));
    }
    let discriminator = &data[0..8];

    match discriminator {
        d if d == Pool::discriminator() => {
            let pool = deserialize_account::<Pool>(data)?;
            println!("{}", pool.custom_format());
        }
        d if d == NftDepositReceipt::discriminator() => {
            let nft_deposit_receipt = deserialize_account::<NftDepositReceipt>(data)?;
            println!("{}", nft_deposit_receipt.custom_format());
        }
        d if d == Whitelist::discriminator() => {
            let whitelist = deserialize_account::<Whitelist>(data)?;
            println!("{}", whitelist.custom_format());
        }
        d if d == WhitelistV2::discriminator() => {
            let whitelist = deserialize_account::<WhitelistV2>(data)?;
            println!("{}", whitelist.custom_format());
        }
        d if d == MintProof::discriminator() => {
            let mint_proof = deserialize_account::<MintProof>(data)?;
            println!("{}", mint_proof.custom_format());
        }
        d if d == MintProofV2::discriminator() => {
            let mint_proof = deserialize_account::<MintProofV2>(data)?;
            println!("{}", mint_proof.custom_format());
        }
        d if d == BidState::discriminator() => {
            let bid_state = deserialize_account::<BidState>(data)?;
            println!("{}", bid_state.custom_format());
        }
        d if d == ListState::discriminator() => {
            let list_state = deserialize_account::<ListState>(data)?;
            println!("{}", list_state.custom_format());
        }
        d if d == OrderState::discriminator() => {
            let order_state = deserialize_account::<OrderState>(data)?;
            println!("{}", order_state.custom_format());
        }
        _ => {
            println!("Unrecognized discriminator");
            println!("Account owned by program: {}", account.owner);
        }
    }
    Ok(())
}

fn is_wallet_type(account: &Account) -> bool {
    account.owner == solana_sdk::system_program::id()
        && account.data.is_empty()
        && !account.executable
}
