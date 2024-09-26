use std::fmt::Display;

use chrono::DateTime;

pub mod amm;
pub mod marketplace;
pub mod price_lock;
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
