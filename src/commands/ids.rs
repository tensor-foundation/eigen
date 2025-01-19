use std::fmt::Display;

use {
    anyhow::Result,
    solana_program::pubkey,
    solana_sdk::pubkey::Pubkey,
    strum::IntoEnumIterator,
    strum_macros::{EnumIter, EnumString},
};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Id {
    #[strum(serialize = "epoch_rewards", serialize = "epochRewards")]
    EpochRewards,
    #[strum(serialize = "last_restart", serialize = "lastRestart")]
    LastRestart,
    #[strum(serialize = "slot_hashes", serialize = "slotHashes")]
    SlotHashes,
    #[strum(serialize = "slot_history", serialize = "slotHistory")]
    SlotHistory,
    #[strum(serialize = "stake_history", serialize = "stakeHistory")]
    StakeHistory,
    #[strum(serialize = "clock", serialize = "sysvar_clock")]
    Clock,
    #[strum(
        serialize = "epoch_schedule",
        serialize = "epochSchedule",
        serialize = "sysvar_epoch_schedule"
    )]
    EpochSchedule,
    #[strum(serialize = "fees", serialize = "sysvar_fees")]
    Fees,
    #[strum(serialize = "instructions", serialize = "sysvar_instructions")]
    Instructions,
    #[strum(serialize = "rent", serialize = "sysvar_rent")]
    Rent,
    #[strum(
        serialize = "wsol",
        serialize = "nativeMint",
        serialize = "native_mint"
    )]
    NativeMint,
    #[strum(serialize = "token", serialize = "spl_token")]
    Token,
    #[strum(
        serialize = "token_2022",
        serialize = "token2022",
        serialize = "spl_token_2022"
    )]
    Token2022,
    #[strum(
        serialize = "token_metadata",
        serialize = "tokenMetadata",
        serialize = "mpl_token_metadata"
    )]
    TokenMetadata,
    #[strum(serialize = "amm", serialize = "tensorAmm", serialize = "tensor_amm")]
    TensorAmm,
    #[strum(
        serialize = "escrow",
        serialize = "tensorEscrow",
        serialize = "tensor_escrow"
    )]
    TensorEscrow,
    #[strum(serialize = "tensor_fees", serialize = "tensorFees")]
    TensorFees,
    #[strum(
        serialize = "market",
        serialize = "tensorMarket",
        serialize = "tensor_market",
        serialize = "marketplace"
    )]
    TensorMarket,
    #[strum(
        serialize = "merkle_tree_config",
        serialize = "merkleTreeConfig",
        serialize = "tensor_merkle_tree_config"
    )]
    TensorMerkleTreeConfig,
    #[strum(
        serialize = "price_lock",
        serialize = "priceLock",
        serialize = "tensor_price_lock"
    )]
    TensorPriceLock,
    #[strum(
        serialize = "whitelist",
        serialize = "tensorWhitelist",
        serialize = "tensor_whitelist"
    )]
    TensorWhitelist,
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
            Id::TensorEscrow => TENSOR_ESCROW_ID,
            Id::TensorFees => TENSOR_FEES_ID,
            Id::TensorMarket => TENSOR_MARKET_ID,
            Id::TensorMerkleTreeConfig => TENSOR_MERKLE_TREE_CONFIG,
            Id::TensorPriceLock => TENSOR_PRICE_LOCK_ID,
            Id::TensorWhitelist => TENSOR_WHITELIST_ID,
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
pub const TENSOR_ESCROW_ID: Pubkey = pubkey!("TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN");
pub const TENSOR_FEES_ID: Pubkey = pubkey!("TFEEgwDP6nn1s8mMX2tTNPPz8j2VomkphLUmyxKm17A");
pub const TENSOR_MARKET_ID: Pubkey = pubkey!("TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp");
pub const TENSOR_MERKLE_TREE_CONFIG: Pubkey =
    pubkey!("4NxSi99mo5hj3BZP6kxWVPgL6skwW6264YNn4LP3X8ML");
pub const TENSOR_PRICE_LOCK_ID: Pubkey = pubkey!("TLoCKic2wGJm7VhZKumih4Lc35fUhYqVMgA4j389Buk");
pub const TENSOR_WHITELIST_ID: Pubkey = pubkey!("TL1ST2iRBzuGTqLn1KXnGdSnEow62BzPnGiqyRXhWtW");

pub fn handle_ids(id: Option<Id>, list: bool) -> Result<()> {
    if list {
        println!("Available IDs:");
        for id in Id::iter() {
            println!("  {}", id);
        }
    } else {
        println!("{}", id.unwrap().get_pubkey());
    }
    Ok(())
}
