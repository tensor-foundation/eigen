use std::fs::File;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_program::pubkey::Pubkey;
use solana_sdk::{account::Account, commitment_config::CommitmentConfig};
use tensor_price_lock::{
    accounts::{OrderNftReceipt, OrderState},
    programs::TENSOR_PRICE_LOCK_ID,
};

use crate::{
    commands::{DEVNET_GENESIS_HASH, MAINNET_GENESIS_HASH},
    discriminators::{deserialize_account, Discriminator},
    setup::CliConfig,
};

pub struct FindPriceLockParams {
    pub rpc_url: Option<String>,
}

struct OrderAccount {
    pubkey: Pubkey,
    account: Account,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct DecodedOrder {
    #[serde_as(as = "DisplayFromStr")]
    pubkey: Pubkey,
    order: OrderState,
}

pub const PL_RECEIPT_DISCRIMINATOR: [u8; 8] = [206, 255, 132, 254, 67, 78, 62, 96];

pub fn handle_find_price_lock(args: FindPriceLockParams) -> Result<()> {
    let cli_config = CliConfig::new(None, args.rpc_url)?;

    let genesis_hash = cli_config.client.get_genesis_hash()?.to_string();

    let cluster = if genesis_hash == MAINNET_GENESIS_HASH {
        "mainnet"
    } else if genesis_hash == DEVNET_GENESIS_HASH {
        "devnet"
    } else {
        "unknown"
    };

    println!("cluster: {}", cluster);

    // GPA to find all Price Lock accounts.

    let gpa_config = RpcProgramAccountsConfig {
        filters: None,
        account_config: RpcAccountInfoConfig {
            data_slice: None,
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            min_context_slot: None,
        },
        with_context: None,
    };

    let accounts: Vec<(Pubkey, Account)> = cli_config
        .client
        .get_program_accounts_with_config(&TENSOR_PRICE_LOCK_ID, gpa_config)?
        .into_iter()
        .collect();

    println!("Found {} price lock accounts", accounts.len());

    for (pubkey, account) in accounts {
        if account.data[..8] != OrderState::discriminator() {
            println!("{}", pubkey);
            println!("{:?}", account.data[..8].to_vec());
        }
    }

    // GPA to find all open price lock orders
    let mut disc = Vec::with_capacity(8);
    disc.extend(OrderState::discriminator());

    let filter = RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Bytes(disc)));
    let filters = vec![filter];

    let gpa_config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            data_slice: None,
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            min_context_slot: None,
        },
        with_context: None,
    };

    let accounts: Vec<OrderAccount> = cli_config
        .client
        .get_program_accounts_with_config(&TENSOR_PRICE_LOCK_ID, gpa_config)?
        .into_iter()
        .map(|(pubkey, account)| OrderAccount { pubkey, account })
        .collect();

    println!("Found {} orders", accounts.len());

    // Get current timestamp
    let slot = cli_config.client.get_slot()?;
    let current_timestamp = cli_config.client.get_block_time(slot)?;

    // Decode orders and find expired ones.
    let decoded_orders: Vec<DecodedOrder> = accounts
        .into_iter()
        .map(|account| {
            let order = deserialize_account::<OrderState>(&account.account.data).ok();
            DecodedOrder {
                pubkey: account.pubkey,
                order: order.unwrap(), // Convert Option<OrderState> to OrderState
            }
        })
        .collect();

    // Write all order pubkeys to a file
    let pubkeys: Vec<String> = decoded_orders
        .iter()
        .map(|o| o.pubkey.to_string())
        .collect();
    let file = File::create(format!("{}_all_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &pubkeys)?;

    // Find orders with a margin value set.
    let margin_orders = decoded_orders
        .iter()
        .filter(|order| order.order.margin.is_some())
        .collect::<Vec<_>>();

    println!(
        "Found {} orders with a margin value set",
        margin_orders.len()
    );

    // Write margin order pubkeys to a file
    let margin_pubkeys: Vec<String> = margin_orders.iter().map(|o| o.pubkey.to_string()).collect();
    let file = File::create(format!("{}_margin_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &margin_pubkeys)?;

    let expired_orders = decoded_orders
        .iter()
        .filter(|order| order.order.expiry < current_timestamp)
        .collect::<Vec<_>>();

    println!("Found {} expired orders", expired_orders.len());

    // Write expired order pubkeys to a file
    let expired_pubkeys: Vec<String> = expired_orders
        .iter()
        .map(|o| o.pubkey.to_string())
        .collect();
    let file = File::create(format!("{}_expired_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &expired_pubkeys)?;

    // Find locked orders
    let locked_orders = decoded_orders
        .iter()
        .filter(|order| order.order.taker.is_some())
        .collect::<Vec<_>>();

    println!("Found {} locked orders", locked_orders.len());

    // Write locked order pubkeys to a file
    let locked_pubkeys: Vec<String> = locked_orders.iter().map(|o| o.pubkey.to_string()).collect();
    let file = File::create(format!("{}_locked_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &locked_pubkeys)?;

    // Find orders with a vault balance
    let vault_orders = decoded_orders
        .iter()
        .filter(|order| order.order.vault_balance > 0)
        .collect::<Vec<_>>();

    println!("Found {} orders with a vault balance", vault_orders.len());

    // Write vault order pubkeys to a file
    let vault_pubkeys: Vec<String> = vault_orders.iter().map(|o| o.pubkey.to_string()).collect();
    let file = File::create(format!("{}_vault_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &vault_pubkeys)?;

    // Find orders with NFTs held
    let nft_orders = decoded_orders
        .iter()
        .filter(|order| order.order.nfts_held == 1)
        .collect::<Vec<_>>();

    println!("Found {} orders with NFTs held", nft_orders.len());

    // Write NFT order pubkeys to a file
    let nft_pubkeys: Vec<String> = nft_orders.iter().map(|o| o.pubkey.to_string()).collect();
    let file = File::create(format!("{}_nft_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &nft_pubkeys)?;

    // Find orders without a vault balance or NFTs held
    let other_orders = decoded_orders
        .iter()
        .filter(|order| order.order.vault_balance == 0 && order.order.nfts_held == 0)
        .collect::<Vec<_>>();

    println!(
        "Found {} orders without a vault balance or NFTs held",
        other_orders.len()
    );

    // Write other order pubkeys to a file
    let other_pubkeys: Vec<String> = other_orders.iter().map(|o| o.pubkey.to_string()).collect();
    let file = File::create(format!("{}_other_orders.json", cluster))?;
    serde_json::to_writer_pretty(file, &other_pubkeys)?;

    Ok(())
}
