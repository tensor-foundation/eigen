use borsh::BorshDeserialize;
use tensor_amm::accounts::{NftDepositReceipt, Pool};
use tensor_marketplace::accounts::{BidState, ListState};
use tensor_price_lock::accounts::OrderState;
use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

use crate::{
    discriminators::deserialize_account,
    formatting::{AccountEntry, CustomFormat},
    types::{
        raydium_clmm::{PoolState as ClmmPoolState, RAYDIUM_CLMM_PROGRAM_ID},
        raydium_cp::{PoolState as CpPoolState, RAYDIUM_CPSWAP_PROGRAM_ID},
        raydium_v4::{AmmInfo, RAYDIUM_AMM_PROGRAM_ID},
    },
    Shard, FEE_SHARDS,
};

use super::*;

pub struct DecodeParams {
    pub rpc_url: Option<String>,
    pub address: Pubkey,
    pub raw: bool,
}

pub fn handle_decode(args: DecodeParams) -> Result<()> {
    let config = CliConfig::new(None, args.rpc_url)?;

    // For `AccountNotFound` error, treat it as an unitialized system program owned wallet with 0 lamports
    // the same way explorers do.
    let account = match config.client.get_account(&args.address) {
        Ok(account) => account,
        Err(e) => {
            // Check if error message contains "AccountNotFound"
            if e.to_string().contains("AccountNotFound") {
                Account {
                    lamports: 0,
                    data: vec![],
                    owner: solana_sdk::system_program::ID,
                    executable: false,
                    rent_epoch: 0,
                }
            } else {
                return Err(e.into());
            }
        }
    };

    if args.raw {
        println!("{:?}", account.data);
        return Ok(());
    }

    if is_fee_shard(&args.address.to_string()) {
        println!(
            "{}",
            Shard {
                address: args.address,
                account
            }
            .custom_format()
        );
        return Ok(());
    }

    if is_wallet_type(&account) {
        let account_entry = AccountEntry {
            address: args.address,
            account,
        };
        println!("{}", account_entry.custom_format());
        return Ok(());
    }

    let mut data = account.data.as_slice();

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
        _ => match account.owner {
            o if o == RAYDIUM_AMM_PROGRAM_ID && data.len() == size_of::<AmmInfo>() => {
                let amm_info = AmmInfo::deserialize(&mut data)?;
                println!("{}", amm_info.custom_format());
            }
            o if o == RAYDIUM_CLMM_PROGRAM_ID => {
                // let clmm_info = ClmmPoolState::deserialize(&mut data)?;
                let clmm_info = deserialize_account::<ClmmPoolState>(data)?;
                println!("{}", clmm_info.custom_format());
            }
            o if o == RAYDIUM_CPSWAP_PROGRAM_ID => {
                // let cpswap_info = CpPoolState::deserialize(&mut data)?;
                let cpswap_info = deserialize_account::<CpPoolState>(data)?;
                println!("{}", cpswap_info.custom_format());
            }
            o if TOKEN_PROGRAM_IDS.contains(&o) => {
                println!("Token or mint account");
                println!("Data length: {}", data.len());
                println!("Lamports: {}", account.lamports);
                println!("Account owned by program: {}", account.owner);
            }
            _ => {
                println!("Unknown account type");
            }
        },
    }
    Ok(())
}

fn is_wallet_type(account: &Account) -> bool {
    account.owner == solana_sdk::system_program::id()
        && account.data.is_empty()
        && !account.executable
}

fn is_fee_shard(address: &str) -> bool {
    FEE_SHARDS.contains(&address)
}
