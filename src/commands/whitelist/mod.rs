mod compare;
mod create;
mod update;

pub use compare::*;
pub use create::*;
pub use update::*;

use std::path::PathBuf;

use crate::{setup::CliConfig, transaction};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use tensor_whitelist::accounts::WhitelistV2;
