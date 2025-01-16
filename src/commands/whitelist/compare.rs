use std::{collections::HashMap, fs::File, path::PathBuf};

use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    serde_with::{serde_as, DisplayFromStr},
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
    },
    solana_program::{pubkey, pubkey::Pubkey},
    solana_sdk::{account::Account, commitment_config::CommitmentConfig},
    tensor_whitelist::{
        accounts::{Whitelist, WhitelistV2},
        programs::TENSOR_WHITELIST_ID,
        types::{Condition, Mode},
    },
};

use crate::{
    discriminators::{deserialize_account, Discriminator},
    formatting::CustomFormat,
    setup::CliConfig,
    spinner::create_spinner,
};

pub const WHITELIST_SIGNER_PUBKEY: Pubkey = pubkey!("DD92UoQnVAaNgRnhvPQhxR7GJkQ9EXhHYq2TEpN8mn1J");

const DEVNET_GENESIS_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";
const MAINNET_GENESIS_HASH: &str = "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";

const DEFAULT_ROOT_HASH: [u8; 32] = [0; 32];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhitelistPair {
    v1_pubkey: Pubkey,
    v1_data: Whitelist,
    v2_pubkey: Pubkey,
    v2_data: Option<WhitelistV2>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MissingWhitelistPair {
    #[serde_as(as = "DisplayFromStr")]
    pub v1_pubkey: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub v2_pubkey: Pubkey,
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
    V2ConditionsLength,
}

pub struct CompareParams {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub list: Option<PathBuf>,
    pub namespace: Option<Pubkey>,
    pub verbose: bool,
}

pub fn handle_compare(args: CompareParams) -> Result<()> {
    let cli_config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let genesis_hash = cli_config.client.get_genesis_hash()?.to_string();

    let namespace = args.namespace.unwrap_or(WHITELIST_SIGNER_PUBKEY);

    let cluster = if genesis_hash == MAINNET_GENESIS_HASH {
        "mainnet"
    } else if genesis_hash == DEVNET_GENESIS_HASH {
        "devnet"
    } else {
        "unknown"
    };

    println!("Fetching whitelists from: {}", cluster);

    // Spinner with empty message we populate later.
    let spinner = create_spinner("")?;

    // Open the list file and decode into a vector of Pubkeys
    let whitelists: Vec<(Pubkey, Account)> = if let Some(list) = args.list {
        spinner.set_message("Opening specified file...");

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
            .map(|account| {
                (
                    Whitelist::find_pda(account.data[8..40].try_into().unwrap()).0,
                    account,
                )
            })
            .collect()
    } else {
        spinner.set_message("Running gPA call to get all whitelist v1s...");

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
    };
    spinner.finish_and_clear();

    println!("Found {} v1 whitelists on-chain", whitelists.len());

    // GPA to find all whitelists v2s
    let mut disc = Vec::with_capacity(8);
    disc.extend(WhitelistV2::discriminator());

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

    let spinner = create_spinner("Running gPA call to get all whitelist v2s...")?;

    let on_chain_whitelist_v2s: HashMap<Pubkey, Account> = cli_config
        .client
        .get_program_accounts_with_config(&TENSOR_WHITELIST_ID, config)?
        .into_iter()
        .collect();

    spinner.finish_and_clear();

    println!(
        "Found {} v2 whitelists on-chain",
        on_chain_whitelist_v2s.len()
    );

    let whitelist_pairs: Vec<WhitelistPair> = whitelists
        .into_iter()
        .map(|(pubkey, account)| {
            (
                pubkey,
                deserialize_account::<Whitelist>(&account.data).unwrap(),
            )
        })
        .map(|(v1_pubkey, v1_data)| {
            let v2_pubkey = WhitelistV2::find_pda(&namespace, v1_data.uuid).0;
            let v2_data = on_chain_whitelist_v2s
                .get(&v2_pubkey)
                .and_then(|account| deserialize_account::<WhitelistV2>(&account.data).ok());

            WhitelistPair {
                v1_pubkey,
                v1_data,
                v2_pubkey,
                v2_data,
            }
        })
        .collect();

    println!("Built pairs");

    // Find missing V2s by finding all the None values in the v2 field
    let (missing_v2s, existing_v2s): (Vec<WhitelistPair>, Vec<WhitelistPair>) = whitelist_pairs
        .into_iter()
        .partition(|pair| pair.v2_data.is_none());

    println!("{} whitelists have no v2 on chain", missing_v2s.len());

    let no_missing_v2s = missing_v2s.is_empty();
    let number_of_missing_v2s = missing_v2s.len();

    let missing_pairs: Vec<MissingWhitelistPair> = missing_v2s
        .into_iter()
        .map(|pair| MissingWhitelistPair {
            v1_pubkey: pair.v1_pubkey,
            v2_pubkey: pair.v2_pubkey,
        })
        .collect();

    let spinner = create_spinner("Writing missing v2s to file...")?;

    // Write v2_missing to a file
    let file = File::create(format!("{}_v2_missing.json", cluster))?;
    serde_json::to_writer_pretty(file, &missing_pairs)?;

    spinner.finish_and_clear();

    // Only compare the whitelists that have a v2 on chain
    let comparison_results = compare_whitelists(&existing_v2s);

    // Write any comparison results with a mismatch to a file. We need to filter out the ones with no mismatch.
    let mismatches = comparison_results
        .iter()
        .filter(|result| result.mismatch.is_some())
        .collect::<Vec<_>>();

    println!(
        "Of the {} whitelists with a v2 on chain, {} have a mismatch",
        existing_v2s.len(),
        mismatches.len()
    );

    let spinner = create_spinner("Writing mismatches to file...")?;

    let file = File::create(format!("{}_mismatches.json", cluster))?;
    serde_json::to_writer_pretty(file, &mismatches)?;

    spinner.finish_and_clear();

    if args.verbose {
        for result in comparison_results.iter() {
            println!("{}", result.custom_format());
            println!(); // Add a blank line between comparisons for readability
        }
    }

    if mismatches.is_empty() && no_missing_v2s {
        println!("All good! ✅ 😎");
    } else {
        println!(
            "There are {} mismatches and {} missing v2s",
            mismatches.len(),
            number_of_missing_v2s
        );
    }

    Ok(())
}

fn has_matching_condition(conditions: &[Condition], mode: Mode, value: &Pubkey) -> bool {
    conditions
        .iter()
        .any(|condition| condition.mode == mode && condition.value == *value)
}

pub fn compare_whitelists(whitelist_pairs: &[WhitelistPair]) -> Vec<ComparisonResult> {
    whitelist_pairs
        .iter()
        .map(|pair| {
            let v1 = &pair.v1_data;
            let v2 = match &pair.v2_data {
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

            if v2.conditions.len() != 1 {
                return ComparisonResult {
                    whitelist_v1: pair.v1_pubkey,
                    whitelist_v2: pair.v2_pubkey,
                    mismatch: Some(Mismatch::V2ConditionsLength),
                };
            }

            ComparisonResult {
                whitelist_v1: pair.v1_pubkey,
                whitelist_v2: pair.v2_pubkey,
                mismatch: None,
            }
        })
        .collect()
}
