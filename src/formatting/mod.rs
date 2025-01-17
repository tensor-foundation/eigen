use std::{fmt::Display, fs::File, io::Write};

use {
    anyhow::Result,
    chrono::DateTime,
    solana_sdk::{account::Account, pubkey::Pubkey},
};

pub mod amm;
pub mod marketplace;
pub mod price_lock;
pub mod raydium;
pub mod wallet;
pub mod whitelist;

pub trait CustomFormat {
    fn custom_format(&self) -> String;
}

pub fn option_formatter<T: Display>(value: &Option<T>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    }
}

pub fn pad_label(label: &str, max_length: usize) -> String {
    format!("{:<width$}", label, width = max_length)
}

pub fn format_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .to_rfc3339()
}

pub struct AccountEntry {
    pub address: Pubkey,
    pub account: Account,
}

pub fn write_formatted<T: CustomFormat>(file_path: &str, items: &[T]) -> Result<()> {
    let mut file = File::create(file_path)?;
    for item in items {
        writeln!(file, "{}\n", item.custom_format())?;
    }
    Ok(())
}
