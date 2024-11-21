use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_program::{pubkey, pubkey::Pubkey};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig};
use tensor_whitelist::{
    accounts::{Whitelist, WhitelistV2},
    programs::TENSOR_WHITELIST_ID,
    types::{Condition, Mode},
};

use crate::{
    discriminators::{deserialize_account, Discriminator},
    formatting::CustomFormat,
    setup::CliConfig,
};

pub const WHITELIST_SIGNER_PUBKEY: Pubkey = pubkey!("Evfeo6yn3ASo3FWkGRKJNfvjF4wCKbuNEkNfYQMtoSBr");

const DEVNET_GENESIS_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";
const MAINNET_GENESIS_HASH: &str = "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";

const DEFAULT_ROOT_HASH: [u8; 32] = [0; 32];

pub struct CompareParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub list: Option<PathBuf>,
    pub verbose: bool,
}

pub fn handle_compare(args: CompareParams) -> Result<()> {
    let cli_config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let genesis_hash = cli_config.client.get_genesis_hash()?.to_string();

    let cluster = if genesis_hash == MAINNET_GENESIS_HASH {
        "mainnet"
    } else if genesis_hash == DEVNET_GENESIS_HASH {
        "devnet"
    } else {
        "unknown"
    };

    println!("cluster: {}", cluster);

    // Open the list file and decode into a vector of Pubkeys
    let whitelists: Vec<Account> = if let Some(list) = args.list {
        let list: Vec<Pubkey> = serde_json::from_reader(File::open(&list)?)?;

        let pubkeys: Vec<_> = list
            .iter()
            .map(|p| Whitelist::find_pda(p.to_bytes()).0)
            .collect();

        cli_config
            .client
            .get_multiple_accounts(&pubkeys)?
            .into_iter()
            .flatten()
            .collect()
    } else {
        // GPA to find all whitelists v1s
        let mut disc = Vec::with_capacity(8);
        disc.extend(Whitelist::discriminator());

        let filter = RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Bytes(disc)));
        let filters = vec![filter];

        let config = RpcProgramAccountsConfig {
            filters: Some(filters),
            account_config: RpcAccountInfoConfig {
                data_slice: None,
                encoding: Some(UiAccountEncoding::Base64),
                commitment: Some(CommitmentConfig::confirmed()),
                min_context_slot: None,
            },
            with_context: None,
        };

        cli_config
            .client
            .get_program_accounts_with_config(&TENSOR_WHITELIST_ID, config)?
            .into_iter()
            .map(|(_, account)| account)
            .collect()
    };

    println!("Found {} v1 whitelists", whitelists.len());

    let mut v1_missing = vec![];

    // Decode whitelist v1.
    let decoded_whitelists: Vec<Option<Whitelist>> = whitelists
        .into_iter()
        .map(|account| {
            Some(account.data).and_then(|data| deserialize_account::<Whitelist>(&data).ok())
        })
        .collect();

    decoded_whitelists.iter().for_each(|w| {
        if w.is_none() {
            v1_missing.push(w);
        }
    });

    println!("missing {} v1 whitelists on chain", v1_missing.len());
    // Write v1_missing to a file
    let file = File::create(format!("{}_v1_missing.json", cluster))?;
    serde_json::to_writer(file, &v1_missing)?;

    let valid_whitelists = decoded_whitelists
        .iter()
        .filter_map(|w| w.as_ref())
        .collect::<Vec<_>>();

    println!("found {} v1 whitelists on chain", valid_whitelists.len());

    // Derive the v2 whitelists PDAs
    let whitelist_v2s = valid_whitelists
        .iter()
        .map(|w| WhitelistV2::find_pda(&WHITELIST_SIGNER_PUBKEY, w.uuid).0)
        .collect::<Vec<_>>();

    // Fetch v2 accounts from on-chain
    let whitelist_v2_accounts = cli_config.client.get_multiple_accounts(&whitelist_v2s)?;

    // Decode v2 accounts
    let decoded_whitelist_v2s: Vec<Option<WhitelistV2>> = whitelist_v2_accounts
        .into_iter()
        .map(|maybe_account| {
            maybe_account.and_then(|a| deserialize_account::<WhitelistV2>(&a.data).ok())
        })
        .collect();

    // Build WhitelistPair structs
    let whitelist_pairs: Vec<WhitelistFullPair> = valid_whitelists
        .iter()
        .zip(whitelist_v2s.iter())
        .zip(decoded_whitelist_v2s.iter())
        .map(|((v1, v2_pubkey), v2_option)| WhitelistFullPair {
            v1_pubkey: Whitelist::find_pda(v1.uuid).0,
            v2_pubkey: *v2_pubkey,
            v1: (*v1).clone(),
            v2: v2_option.clone(),
        })
        .collect();

    // Find missing V2s by finding all the None values in the v2 field
    let missing_v2s: Vec<WhitelistPair> = whitelist_pairs
        .iter()
        .filter(|pair| pair.v2.is_none())
        .map(|pair| WhitelistPair {
            v1_pubkey: pair.v1_pubkey,
            v2_pubkey: pair.v2_pubkey,
        })
        .collect();

    println!("{} whitelists have no v2 on chain", missing_v2s.len());
    println!("{:?}", missing_v2s);

    // Write v2_missing to a file
    let file = File::create(format!("{}_v2_missing.json", cluster))?;
    serde_json::to_writer_pretty(file, &missing_v2s)?;

    let comparison_results = compare_whitelists(&whitelist_pairs);

    // Write any comparison results with a mismatch to a file. We need to filter out the ones with no mismatch.
    let mismatches = comparison_results
        .iter()
        .filter(|result| result.mismatch.is_some())
        .collect::<Vec<_>>();

    println!("{} mismatches found", mismatches.len());

    let file = File::create(format!("{}_mismatches.json", cluster))?;
    serde_json::to_writer_pretty(file, &mismatches)?;

    if args.verbose {
        for result in comparison_results {
            println!("{}", result.custom_format());
            println!(); // Add a blank line between comparisons for readability
        }
    }

    Ok(())
}

use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhitelistFullPair {
    #[serde_as(as = "DisplayFromStr")]
    pub v1_pubkey: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub v2_pubkey: Pubkey,
    pub v1: Whitelist,
    pub v2: Option<WhitelistV2>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhitelistPair {
    #[serde_as(as = "DisplayFromStr")]
    pub v1_pubkey: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub v2_pubkey: Pubkey,
}
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct V2Missing {
    #[serde_as(as = "DisplayFromStr")]
    pub whitelist_v1: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub whitelist_v2: Pubkey,
    pub v2_exists: bool,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComparisonResult {
    #[serde_as(as = "DisplayFromStr")]
    pub whitelist_v1: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub whitelist_v2: Pubkey,
    pub mismatch: Option<Mismatch>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Mismatch {
    Uuid,
    MerkleRoot,
    Voc,
    Fvc,
    V2Missing,
}

fn has_matching_condition(conditions: &[Condition], mode: Mode, value: &Pubkey) -> bool {
    conditions
        .iter()
        .any(|condition| condition.mode == mode && condition.value == *value)
}

pub fn compare_whitelists(whitelist_pairs: &[WhitelistFullPair]) -> Vec<ComparisonResult> {
    whitelist_pairs
        .iter()
        .map(|pair| {
            let v1 = &pair.v1;
            let v2 = match &pair.v2 {
                Some(v2) => v2,
                None => {
                    return ComparisonResult {
                        whitelist_v1: pair.v1_pubkey,
                        whitelist_v2: pair.v2_pubkey,
                        mismatch: Some(Mismatch::V2Missing),
                    }
                }
            };

            if v1.uuid != v2.uuid {
                return ComparisonResult {
                    whitelist_v1: pair.v1_pubkey,
                    whitelist_v2: pair.v2_pubkey,
                    mismatch: Some(Mismatch::Uuid),
                };
            }

            if v1.root_hash != DEFAULT_ROOT_HASH
                && !has_matching_condition(
                    &v2.conditions,
                    Mode::MerkleTree,
                    &Pubkey::new_from_array(v1.root_hash),
                )
            {
                return ComparisonResult {
                    whitelist_v1: pair.v1_pubkey,
                    whitelist_v2: pair.v2_pubkey,
                    mismatch: Some(Mismatch::MerkleRoot),
                };
            }

            if let Some(voc) = v1.voc {
                if !v2
                    .conditions
                    .iter()
                    .any(|condition| matches!(condition.mode, Mode::VOC) && condition.value == voc)
                {
                    return ComparisonResult {
                        whitelist_v1: pair.v1_pubkey,
                        whitelist_v2: pair.v2_pubkey,
                        mismatch: Some(Mismatch::Voc),
                    };
                }
            }

            if let Some(fvc) = v1.fvc {
                if !v2
                    .conditions
                    .iter()
                    .any(|condition| matches!(condition.mode, Mode::FVC) && condition.value == fvc)
                {
                    return ComparisonResult {
                        whitelist_v1: pair.v1_pubkey,
                        whitelist_v2: pair.v2_pubkey,
                        mismatch: Some(Mismatch::Fvc),
                    };
                }
            }

            ComparisonResult {
                whitelist_v1: pair.v1_pubkey,
                whitelist_v2: pair.v2_pubkey,
                mismatch: None,
            }
        })
        .collect()
}
