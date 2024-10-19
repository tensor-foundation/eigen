mod decode;
mod download;
mod eigen;
mod error;
mod fees;
mod pool;
mod whitelist;

pub use decode::*;
pub use download::*;
pub use eigen::*;
pub use error::*;
pub use fees::*;
pub use pool::*;
pub use whitelist::*;

pub use crate::{discriminators::Discriminator, setup::CliConfig, transaction};

use anyhow::{anyhow, Result};
use solana_sdk::{account::Account, pubkey::Pubkey, signer::Signer, transaction::Transaction};
use std::path::PathBuf;

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
