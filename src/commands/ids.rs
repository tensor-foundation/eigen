use std::str::FromStr;

use {anyhow::Result, solana_program::pubkey, solana_sdk::pubkey::Pubkey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Id {
    EpochRewards,
    LastRestart,
    SlotHashes,
    SlotHistory,
    StakeHistory,
    Clock,
    EpochSchedule,
    Fees,
    Instructions,
    Rent,
    NativeMint,
    Token,
    Token2022,
    TokenMetadata,
    TensorAmm,
}

fn parse_id_string(s: &str) -> String {
    s.replace(['-', ' '], "_").to_lowercase()
}

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_id_string(s).as_str() {
            "epoch_rewards" => Ok(Id::EpochRewards),
            "last_restart" => Ok(Id::LastRestart),
            "slot_hashes" => Ok(Id::SlotHashes),
            "slot_history" => Ok(Id::SlotHistory),
            "stake_history" => Ok(Id::StakeHistory),
            "sysvar_clock" | "clock" => Ok(Id::Clock),
            "sysvar_epoch_schedule" | "epoch_schedule" => Ok(Id::EpochSchedule),
            "sysvar_fees" | "fees" => Ok(Id::Fees),
            "sysvar_instructions" | "instructions" => Ok(Id::Instructions),
            "sysvar_rent" | "rent" => Ok(Id::Rent),
            "wsol" | "native_mint" => Ok(Id::NativeMint),
            "spl_token" | "token" => Ok(Id::Token),
            "spl_token_2022" | "token_2022" => Ok(Id::Token2022),
            "mpl_token_metadata" | "token_metadata" => Ok(Id::TokenMetadata),
            "tensor_amm" | "amm" => Ok(Id::TensorAmm),
            _ => Err(anyhow::anyhow!("Invalid ID name: {}", s)),
        }
    }
}

impl Id {
    pub fn get_pubkey(&self) -> Pubkey {
        match self {
            Id::EpochRewards => EPOCH_REWARDS,
            Id::LastRestart => LAST_RESTART,
            Id::SlotHashes => SLOT_HASHES,
            Id::SlotHistory => SLOT_HISTORY,
            Id::StakeHistory => STAKE_HISTORY,
            Id::Clock => SYSVAR_CLOCK,
            Id::EpochSchedule => SYVAR_EPOCH_SCHEDULE,
            Id::Fees => SYSVAR_FEES,
            Id::Instructions => SYSVAR_INSTRUCTIONS,
            Id::Rent => SYSVAR_RENT,
            Id::NativeMint => NATIVE_MINT_ID,
            Id::Token => TOKEN_ID,
            Id::Token2022 => TOKEN_2022_ID,
            Id::TokenMetadata => TOKEN_METADATA_ID,
            Id::TensorAmm => TENSOR_AMM_ID,
        }
    }
}

// System Variables
pub const EPOCH_REWARDS: Pubkey = pubkey!("SysvarEpochRewards1111111111111111111111111");
pub const LAST_RESTART: Pubkey = pubkey!("SysvarLastRestartS1ot1111111111111111111111");
pub const SLOT_HASHES: Pubkey = pubkey!("SysvarS1otHashes111111111111111111111111111");
pub const SLOT_HISTORY: Pubkey = pubkey!("SysvarS1otHistory11111111111111111111111111");
pub const STAKE_HISTORY: Pubkey = pubkey!("SysvarStakeHistory1111111111111111111111111");
pub const SYSVAR_CLOCK: Pubkey = pubkey!("SysvarC1ock11111111111111111111111111111111");
pub const SYVAR_EPOCH_SCHEDULE: Pubkey = pubkey!("SysvarEpochSchedu1e111111111111111111111111");
pub const SYSVAR_FEES: Pubkey = pubkey!("SysvarFees111111111111111111111111111111111");
pub const SYSVAR_INSTRUCTIONS: Pubkey = pubkey!("Sysvar1nstructions1111111111111111111111111");
pub const SYSVAR_RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

// SPL
pub const NATIVE_MINT_ID: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_2022_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

// Metaplex
pub const TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

// Tensor Foundation
pub const TENSOR_AMM_ID: Pubkey = pubkey!("TAMM6ub33ij1mbetoMyVBLeKY5iP41i4UPUJQGkhfsg");

pub fn handle_ids(id: Id) -> Result<()> {
    println!("{}", id.get_pubkey());
    Ok(())
}
