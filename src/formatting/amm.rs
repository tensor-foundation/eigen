use console::Style;
use tensor_amm::{
    accounts::{NftDepositReceipt, Pool},
    types::{PoolConfig, PoolStats},
};

use crate::formatting::{format_timestamp, pad_label};

use super::CustomFormat;

const POOL_LABEL_LENGTH: usize = 25;
const POOL_SUB_LABEL_LENGTH: usize = POOL_LABEL_LENGTH - 2;
const RECEIPT_LABEL_LENGTH: usize = 18;
const LINE_BREAK: &str = "-------------------------";

impl CustomFormat for Pool {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}
{}
{}: {}
{}: {}
{}: {}
{}: {}
{}
{}: {}",
            color.apply_to("Pool---------------------"),
            pad_label("discriminator", POOL_LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("version", POOL_LABEL_LENGTH),
            color.apply_to(self.version),
            pad_label("bump", POOL_LABEL_LENGTH),
            color.apply_to(self.bump[0]),
            pad_label("pool_id", POOL_LABEL_LENGTH),
            color.apply_to(String::from_utf8_lossy(&self.pool_id)),
            pad_label("created_at", POOL_LABEL_LENGTH),
            color.apply_to(format_timestamp(self.created_at)),
            pad_label("updated_at", POOL_LABEL_LENGTH),
            color.apply_to(format_timestamp(self.updated_at)),
            pad_label("expiry", POOL_LABEL_LENGTH),
            color.apply_to(format_timestamp(self.expiry)),
            pad_label("owner", POOL_LABEL_LENGTH),
            color.apply_to(self.owner),
            pad_label("whitelist", POOL_LABEL_LENGTH),
            color.apply_to(self.whitelist),
            pad_label("rent_payer", POOL_LABEL_LENGTH),
            color.apply_to(self.rent_payer),
            pad_label("currency", POOL_LABEL_LENGTH),
            color.apply_to(self.currency),
            pad_label("amount", POOL_LABEL_LENGTH),
            color.apply_to(self.amount),
            pad_label("price_offset", POOL_LABEL_LENGTH),
            color.apply_to(self.price_offset),
            pad_label("nfts_held", POOL_LABEL_LENGTH),
            color.apply_to(self.nfts_held),
            self.stats.custom_format(),
            pad_label("shared_escrow", POOL_LABEL_LENGTH),
            color.apply_to(self.shared_escrow),
            pad_label("cosigner", POOL_LABEL_LENGTH),
            color.apply_to(self.cosigner),
            pad_label("maker_broker", POOL_LABEL_LENGTH),
            color.apply_to(self.maker_broker),
            pad_label("max_taker_sell_count", POOL_LABEL_LENGTH),
            color.apply_to(self.max_taker_sell_count),
            self.config.custom_format(),
            pad_label("reserved", POOL_LABEL_LENGTH),
            if self.reserved.iter().all(|&x| x == 0) {
                color.apply_to("[all zeros]".to_string())
            } else {
                color.apply_to(format!("{:?}", &self.reserved[..]))
            }
        )
    }
}

impl CustomFormat for PoolConfig {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();
        format!(
            "{}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
  {}: {}
{}",
            color.apply_to("--PoolConfig--------------"),
            pad_label("pool_type", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.pool_type),
            pad_label("curve_type", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.curve_type),
            pad_label("starting_price", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.starting_price),
            pad_label("delta", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.delta),
            pad_label("mm_compound_fees", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.mm_compound_fees),
            pad_label("mm_fee_bps", POOL_SUB_LABEL_LENGTH),
            color.apply_to(
                self.mm_fee_bps
                    .to_option()
                    .map_or("None".to_string(), |bps| bps.to_string())
            ),
            color.apply_to(LINE_BREAK)
        )
    }
}

impl CustomFormat for PoolStats {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();
        format!(
            "{}
  {}: {}
  {}: {}
  {}: {}
{}",
            color.apply_to("--PoolStats-------------"),
            pad_label("taker_sell_count", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.taker_sell_count),
            pad_label("taker_buy_count", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.taker_buy_count),
            pad_label("accumulated_mm_profit", POOL_SUB_LABEL_LENGTH),
            color.apply_to(self.accumulated_mm_profit),
            color.apply_to(LINE_BREAK)
        )
    }
}

impl CustomFormat for NftDepositReceipt {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();
        format!(
            "{}
{}: {}
{}: {}
{}: {}",
            color.apply_to("NftDepositReceipt"),
            pad_label("bump", RECEIPT_LABEL_LENGTH),
            color.apply_to(self.bump),
            pad_label("mint", RECEIPT_LABEL_LENGTH),
            color.apply_to(self.mint),
            pad_label("pool", RECEIPT_LABEL_LENGTH),
            color.apply_to(self.pool)
        )
    }
}
