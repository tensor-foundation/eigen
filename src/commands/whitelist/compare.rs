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
    formatting::{write_formatted, CustomFormat},
    setup::CliConfig,
    spinner::create_spinner,
};

pub const WHITELIST_SIGNER_PUBKEY: Pubkey = pubkey!("DD92UoQnVAaNgRnhvPQhxR7GJkQ9EXhHYq2TEpN8mn1J");

const DEVNET_GENESIS_HASH: &str = "EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG";
const MAINNET_GENESIS_HASH: &str = "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d";

const DEFAULT_ROOT_HASH: [u8; 32] = [0; 32];

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhitelistPair {
    #[serde_as(as = "DisplayFromStr")]
    pub v1_pubkey: Pubkey,
    pub v1_data: Whitelist,
    #[serde_as(as = "DisplayFromStr")]
    pub v2_pubkey: Pubkey,
    pub v2_data: Option<WhitelistV2>,
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Mismatch {
    Uuid,
    MerkleRoot,
    Voc,
    Fvc,
    V2Missing,
    V2ConditionsLength,
    UnexpectedV2Conditions,
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

    let spinner = create_spinner("Writing successful matches to file...")?;
    let file = File::create(format!("{}_successful_matches.json", cluster))?;
    serde_json::to_writer_pretty(file, &existing_v2s)?;

    write_formatted(
        &format!("{}_v2_successful_matches.txt", cluster),
        &existing_v2s,
    )?;
    spinner.finish_and_clear();

    // Only compare the whitelists that have a v2 on chain
    let comparison_results = compare_whitelists(&existing_v2s);

    // Write any comparison results with a mismatch to a file. We need to filter out the ones with no mismatch.
    let mismatches = comparison_results
        .iter()
        .filter(|result| result.mismatch.is_some())
        .collect::<Vec<_>>();

    println!(
        "Of the {} whitelist v1s with a v2 on chain, {} have a mismatch",
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
            let mut result = ComparisonResult {
                whitelist_v1: pair.v1_pubkey,
                whitelist_v2: pair.v2_pubkey,
                mismatch: None,
            };

            let v1 = &pair.v1_data;
            let v2 = match &pair.v2_data {
                Some(v2) => v2,
                None => {
                    result.mismatch = Some(Mismatch::V2Missing);
                    return result;
                }
            };

            // Check UUID
            if v1.uuid != v2.uuid {
                result.mismatch = Some(Mismatch::Uuid);
                return result;
            }

            // Check conditions length
            if v2.conditions.len() != 1 {
                result.mismatch = Some(Mismatch::V2ConditionsLength);
                return result;
            }

            // Return early if we find a matching condition
            if v1.root_hash != DEFAULT_ROOT_HASH {
                if has_matching_condition(
                    &v2.conditions,
                    Mode::MerkleTree,
                    &Pubkey::new_from_array(v1.root_hash),
                ) {
                    return result;
                } else {
                    result.mismatch = Some(Mismatch::MerkleRoot);
                    return result;
                }
            }

            if let Some(voc) = v1.voc {
                if has_matching_condition(&v2.conditions, Mode::VOC, &voc) {
                    return result;
                } else {
                    result.mismatch = Some(Mismatch::Voc);
                    return result;
                }
            }

            if let Some(fvc) = v1.fvc {
                if has_matching_condition(&v2.conditions, Mode::FVC, &fvc) {
                    return result;
                } else {
                    result.mismatch = Some(Mismatch::Fvc);
                    return result;
                }
            }

            result.mismatch = Some(Mismatch::UnexpectedV2Conditions);
            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;
    use tensor_whitelist::types::{Condition, Mode, State};

    fn create_whitelist(
        uuid: [u8; 32],
        root_hash: [u8; 32],
        voc: Option<Pubkey>,
        fvc: Option<Pubkey>,
    ) -> Whitelist {
        Whitelist {
            discriminator: [0; 8],
            version: 0,
            bump: 0,
            verified: false,
            root_hash,
            uuid,
            name: [0; 32],
            frozen: false,
            voc,
            fvc,
            reserved: [0; 64],
        }
    }

    fn create_whitelist_v2(
        uuid: [u8; 32],
        conditions: Vec<Condition>,
        namespace: Pubkey,
    ) -> WhitelistV2 {
        WhitelistV2 {
            discriminator: [0; 8],
            version: 0,
            bump: 0,
            state: State::Unfrozen,
            update_authority: Pubkey::new_unique(),
            namespace,
            freeze_authority: Pubkey::new_unique(),
            conditions,
            uuid,
        }
    }

    #[test]
    fn test_merkle_whitelist() {
        // Test a whitelist v1 with merkle root set, and corresponding v2 with matching condition

        let uuid = [1u8; 32];

        // Set root_hash to some non-default value
        let root_hash: [u8; 32] = [1; 32];

        let namespace = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, root_hash, None, None);

        let condition = Condition {
            mode: Mode::MerkleTree,
            value: Pubkey::new_from_array(root_hash),
        };
        let v2_data = create_whitelist_v2(uuid, vec![condition], namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        assert!(results[0].mismatch.is_none());
    }

    #[test]
    fn test_voc_whitelist() {
        // Test a whitelist v1 with VOC set, and corresponding v2 with matching condition

        let uuid = [2u8; 32];
        let namespace = Pubkey::new_unique();

        let voc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, Some(voc), None);

        let condition = Condition {
            mode: Mode::VOC,
            value: voc,
        };
        let v2_data = create_whitelist_v2(uuid, vec![condition], namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        assert!(results[0].mismatch.is_none());
    }

    #[test]
    fn test_fvc_whitelist() {
        // Test a whitelist v1 with FVC set, and corresponding v2 with matching condition

        let uuid = [3u8; 32];
        let namespace = Pubkey::new_unique();

        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, None, Some(fvc));

        let condition = Condition {
            mode: Mode::FVC,
            value: fvc,
        };
        let v2_data = create_whitelist_v2(uuid, vec![condition], namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        assert!(results[0].mismatch.is_none());
    }

    #[test]
    fn test_voc_and_fvc_set() {
        // Test a whitelist v1 with both VOC and FVC set

        let uuid = [4u8; 32];
        let namespace = Pubkey::new_unique();

        let voc = Pubkey::new_unique();
        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, Some(voc), Some(fvc));

        // Test with v2 condition matching VOC
        let condition_voc = Condition {
            mode: Mode::VOC,
            value: voc,
        };
        let v2_data_voc = create_whitelist_v2(uuid, vec![condition_voc], namespace);

        let pair_voc = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data: v1_data.clone(),
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data_voc.uuid).0,
            v2_data: Some(v2_data_voc),
        };

        let results_voc = compare_whitelists(&[pair_voc]);

        assert_eq!(results_voc.len(), 1);
        assert!(results_voc[0].mismatch.is_none());

        // Test with v2 condition matching FVC
        let condition_fvc = Condition {
            mode: Mode::FVC,
            value: fvc,
        };
        let v2_data_fvc = create_whitelist_v2(uuid, vec![condition_fvc], namespace);

        let pair_fvc = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data: v1_data.clone(),
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data_fvc.uuid).0,
            v2_data: Some(v2_data_fvc),
        };

        let results_fvc = compare_whitelists(&[pair_fvc]);

        assert_eq!(results_fvc.len(), 1);
        // Even though v2 has a condition matching FVC, it should still mismatch Voc is set on v1
        // and takes priority over FVC.
        assert_eq!(results_fvc[0].mismatch, Some(Mismatch::Voc));
    }

    #[test]
    fn test_merkle_and_fvc_set() {
        // Test a whitelist v1 with Merkle root and FVC set

        let uuid = [5u8; 32];
        let namespace = Pubkey::new_unique();

        let root_hash: [u8; 32] = [2; 32];
        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, root_hash, None, Some(fvc));

        // Test with v2 condition matching Merkle root
        let condition_merkle = Condition {
            mode: Mode::MerkleTree,
            value: Pubkey::new_from_array(root_hash),
        };
        let v2_data_merkle = create_whitelist_v2(uuid, vec![condition_merkle], namespace);

        let pair_merkle = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data: v1_data.clone(),
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data_merkle.uuid).0,
            v2_data: Some(v2_data_merkle),
        };

        let results_merkle = compare_whitelists(&[pair_merkle]);

        assert_eq!(results_merkle.len(), 1);
        assert!(results_merkle[0].mismatch.is_none());

        // Test with v2 condition matching FVC
        let condition_fvc = Condition {
            mode: Mode::FVC,
            value: fvc,
        };
        let v2_data_fvc = create_whitelist_v2(uuid, vec![condition_fvc], namespace);

        let pair_fvc = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data_fvc.uuid).0,
            v2_data: Some(v2_data_fvc),
        };

        let results_fvc = compare_whitelists(&[pair_fvc]);

        assert_eq!(results_fvc.len(), 1);
        // Even though v2 has a condition matching FVC, it should still mismatch Merklet is set
        // on v1 and takes priority.
        assert_eq!(results_fvc[0].mismatch, Some(Mismatch::MerkleRoot));
    }

    #[test]
    fn test_voc_with_multiple_v2_conditions() {
        // v1 has VOC set but v2 has two conditions (VOC and FVC)
        // Assert V2 conditions length mismatch

        let uuid = [6u8; 32];
        let namespace = Pubkey::new_unique();

        let voc = Pubkey::new_unique();
        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, Some(voc), None);

        let conditions = vec![
            Condition {
                mode: Mode::VOC,
                value: voc,
            },
            Condition {
                mode: Mode::FVC,
                value: fvc,
            },
        ];

        let v2_data = create_whitelist_v2(uuid, conditions, namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        // Expect a V2ConditionsLength mismatch because v2 has more than one condition
        assert_eq!(results[0].mismatch, Some(Mismatch::V2ConditionsLength));
    }

    #[test]
    fn test_voc_and_fvc_v1_v2_has_voc() {
        // v1 has VOC and FVC set, v2 has just VOC set
        // Assert mismatch is None (successful match)

        let uuid = [7u8; 32];
        let namespace = Pubkey::new_unique();

        let voc = Pubkey::new_unique();
        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, Some(voc), Some(fvc));

        let condition = Condition {
            mode: Mode::VOC,
            value: voc,
        };
        let v2_data = create_whitelist_v2(uuid, vec![condition], namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        // Expect no mismatch since v2 matches one of the v1 conditions
        assert!(results[0].mismatch.is_none());
    }

    #[test]
    fn test_merkle_with_multiple_v2_conditions() {
        // v1 has Merkle root set, v2 has Merkle and FVC conditions
        // Assert V2 conditions length mismatch

        let uuid = [8u8; 32];
        let namespace = Pubkey::new_unique();

        let root_hash: [u8; 32] = [3; 32];
        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, root_hash, None, None);

        let conditions = vec![
            Condition {
                mode: Mode::MerkleTree,
                value: Pubkey::new_from_array(root_hash),
            },
            Condition {
                mode: Mode::FVC,
                value: fvc,
            },
        ];

        let v2_data = create_whitelist_v2(uuid, conditions, namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        // Expect a V2ConditionsLength mismatch because v2 has more than one condition
        assert_eq!(results[0].mismatch, Some(Mismatch::V2ConditionsLength));
    }

    #[test]
    fn test_v1_no_conditions_v2_has_fvc() {
        // v1 has none of the conditions set, v2 has FVC condition
        // Assert mismatch is UnexpectedV2Conditions

        let uuid = [9u8; 32];
        let namespace = Pubkey::new_unique();

        let fvc = Pubkey::new_unique();

        let v1_data = create_whitelist(uuid, DEFAULT_ROOT_HASH, None, None);

        let condition = Condition {
            mode: Mode::FVC,
            value: fvc,
        };
        let v2_data = create_whitelist_v2(uuid, vec![condition], namespace);

        let pair = WhitelistPair {
            v1_pubkey: Whitelist::find_pda(v1_data.uuid).0,
            v1_data,
            v2_pubkey: WhitelistV2::find_pda(&namespace, v2_data.uuid).0,
            v2_data: Some(v2_data),
        };

        let results = compare_whitelists(&[pair]);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].mismatch, Some(Mismatch::UnexpectedV2Conditions));
    }
}
