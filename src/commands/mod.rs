mod decode;
mod derive;
mod download;
mod eigen;
mod error;
mod fees;
mod pool;
mod whitelist;

pub use decode::*;
pub use derive::*;
pub use download::*;
pub use eigen::*;
pub use error::*;
pub use fees::*;
pub use pool::*;
pub use whitelist::*;

pub use crate::{discriminators::Discriminator, setup::CliConfig, transaction};

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use solana_sdk::{account::Account, pubkey::Pubkey};

#[macro_export]
macro_rules! transaction {
    ($signers:expr, $instructions:expr, $client:expr) => {
        Transaction::new_signed_with_payer(
            $instructions,
            Some(&$signers[0].pubkey()),
            $signers,
            $client.get_latest_blockhash()?,
        )
    };
}

pub const fn pubkey(base58str: &str) -> Pubkey {
    Pubkey::new_from_array(five8_const::decode_32_const(base58str))
}

pub const TOKEN_PROGRAM_IDS: &[Pubkey] = &[
    pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    pubkey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
];
