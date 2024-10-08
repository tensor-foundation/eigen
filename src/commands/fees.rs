use super::*;

use std::{fs::File, str::FromStr};

use solana_sdk::{
    instruction::Instruction, pubkey, signer::Signer, system_instruction, transaction::Transaction,
};

const TFEE_PROGRAM_ID: Pubkey = pubkey!("TFEEgwDP6nn1s8mMX2tTNPPz8j2VomkphLUmyxKm17A");

pub struct FeeArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
}

pub fn generate_fee_shards() -> Result<()> {
    let mut shards = vec![];

    for i in 0..255 {
        let shard: &[u8] = &[i];
        shards.push(
            Pubkey::find_program_address(&[b"fee_shard", shard], &TFEE_PROGRAM_ID)
                .0
                .to_string(),
        );
    }

    let file = File::create("fee_shards.json")?;
    serde_json::to_writer_pretty(&file, &shards)?;

    Ok(())
}

pub fn fund_shards(args: FeeArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let rent_exempt_lamports = config.client.get_minimum_balance_for_rent_exemption(0)?;

    // Read fee shards from file
    let file = File::open("fee_shards.json")?;
    let shards: Vec<String> = serde_json::from_reader(file)?;

    // Convert shards to Pubkeys
    let shard_pubkeys: Vec<Pubkey> = shards
        .iter()
        .filter_map(|s| Pubkey::from_str(s).ok())
        .collect();

    // Check balances and create transfer instructions only for underfunded shards
    let mut instructions: Vec<Instruction> = Vec::new();
    for pubkey in &shard_pubkeys {
        let balance = config.client.get_balance(pubkey)?;
        if balance < rent_exempt_lamports {
            instructions.push(system_instruction::transfer(
                &config.keypair.pubkey(),
                pubkey,
                rent_exempt_lamports - balance,
            ));
        }
    }

    // Pack instructions into transactions (15 instructions per transaction)
    for chunk in instructions.chunks(15) {
        let transaction = Transaction::new_signed_with_payer(
            chunk,
            Some(&config.keypair.pubkey()),
            &[&config.keypair],
            config.client.get_latest_blockhash()?,
        );

        config
            .client
            .send_and_confirm_transaction_with_spinner(&transaction)?;
    }

    Ok(())
}

pub fn get_shard_balances(args: FeeArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let file = File::open("fee_shards.json")?;
    let shards: Vec<String> = serde_json::from_reader(file)?;

    let shard_pubkeys: Vec<Pubkey> = shards
        .iter()
        .filter_map(|s| Pubkey::from_str(s).ok())
        .collect();

    let rent_exempt_lamports = config.client.get_minimum_balance_for_rent_exemption(0)?;

    for pubkey in shard_pubkeys {
        let balance = config.client.get_balance(&pubkey)?;
        let status = if balance == rent_exempt_lamports {
            "✓"
        } else {
            "✗"
        };
        println!("{} {} {}", pubkey, balance, status);
    }

    Ok(())
}
