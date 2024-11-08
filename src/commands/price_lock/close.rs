use anyhow::Result;
use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::fs::{self, File};
use std::path::PathBuf;

use crate::{commands::pubkey, setup::CliConfig};

const TOKEN_2022_ID: Pubkey = pubkey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

pub struct ClosePriceLockParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct RollState {
    pub roll_state: Pubkey,                //32
    pub user: Pubkey,                      //32
    pub user_nonce: [u8; 32],              //32
    pub secret_hash: [u8; 32],             //32
    pub wager: u64,                        //8
    pub roll_count: u64,                   //8
    pub commit_slot: u64,                  //8
    pub optional_royalty_pct: Option<u16>, //4
    pub version: u8,                       //1
    pub bump: [u8; 1],                     //1
    /// storing the bump makes fn buyer_seeds possible without upsetting the borrow checker
    pub buyer_bump: [u8; 1], //1
    pub _reserved0: [u8; 1],               //1
    pub _reserved1: [u8; 128],             //128
    pub requested_rewards: Vec<RequestedReward>, //4 + len * 200
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct RequestedReward {
    /// all in TCOMP/TSWAP price
    pub post_market_fees_amount: u64, //8
    pub odds_bps: u16,        //4
    pub _reserved0: [u8; 4],  //4
    pub reward: Reward,       //120
    pub _reserved1: [u8; 64], //64
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct RequestedRewardArg {
    pub odds_bps: u16,
    pub reward: Reward,
}

#[derive(BorshDeserialize, Clone, Debug)]
pub enum Reward {
    None,
    TCompListing { details: RewardDetails },
    TSwapPool { details: RewardDetails },
    TAmmPool { details: RewardDetails },
    TSwapListing { details: RewardDetails },
}

#[derive(BorshDeserialize, Clone, Debug)]
pub struct RewardDetails {
    /// Listing or Pool
    pub address: Pubkey,
    pub mint: Pubkey,
    /// Listing or Pool owner
    pub owner: Pubkey,
    pub payment_mint: Option<Pubkey>,
    pub token_standard: Option<TokenStandardLocal>,
    /// base amount before TCOMP/TSWAP fees and royalties
    pub payment_base_amount: u64,
    /// TCOMP parameter
    pub royalty_bps: u16,
    /// TAmm parameter
    pub mm_fee_bps: u64,
}

#[derive(BorshDeserialize, Clone, Debug)]
pub enum TokenStandardLocal {
    NonFungible,                    // This is a master edition
    FungibleAsset,                  // A token with metadata that can also have attrributes
    Fungible,                       // A token with simple metadata
    NonFungibleEdition,             // This is a limited edition
    ProgrammableNonFungible,        // NonFungible with programmable configuration
    ProgrammableNonFungibleEdition, // NonFungible edition with programmable configuration
}

pub fn handle_close_price_lock(args: ClosePriceLockParams) -> Result<()> {
    let cli_config = CliConfig::new(None, args.rpc_url)?;

    let pubkey = pubkey("9ujeuLhKRDN82cYgmDqquiXkkY8gLP2Zgu7jKfJiZXn6");

    let roll_state = cli_config.client.get_account(&pubkey)?;
    let roll_state: RollState = RollState::deserialize(&mut roll_state.data.as_slice())?;

    println!("{:?}", roll_state);

    Ok(())
}
