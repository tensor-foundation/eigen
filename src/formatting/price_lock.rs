use console::Style;
use tensor_price_lock::accounts::{OrderNftReceipt, OrderState};

use crate::formatting::{format_timestamp, option_formatter, pad_label};

use super::CustomFormat;

const LABEL_LENGTH: usize = 25;

impl CustomFormat for OrderState {
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
{}: {}
{}: {}
{}: {}
{}: {}",
            color.apply_to("OrderState---------------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("version", LABEL_LENGTH),
            color.apply_to(self.version),
            pad_label("bump", LABEL_LENGTH),
            color.apply_to(self.bump[0]),
            pad_label("order_id", LABEL_LENGTH),
            color.apply_to(hex::encode(self.order_id)),
            pad_label("order_type", LABEL_LENGTH),
            color.apply_to(format!("{:?}", self.order_type)),
            pad_label("nonce", LABEL_LENGTH),
            color.apply_to(self.nonce),
            pad_label("maker", LABEL_LENGTH),
            color.apply_to(self.maker),
            pad_label("price", LABEL_LENGTH),
            color.apply_to(self.price),
            pad_label("currency", LABEL_LENGTH),
            color.apply_to(option_formatter(&self.currency)),
            pad_label("apr_bps", LABEL_LENGTH),
            color.apply_to(self.apr_bps),
            pad_label("duration_sec", LABEL_LENGTH),
            color.apply_to(self.duration_sec),
            pad_label("whitelist", LABEL_LENGTH),
            color.apply_to(self.whitelist),
            pad_label("maker_broker", LABEL_LENGTH),
            color.apply_to(option_formatter(&self.maker_broker)),
            pad_label("margin", LABEL_LENGTH),
            color.apply_to(option_formatter(&self.margin)),
            pad_label("expiry", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.expiry)),
            pad_label("created_at", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.created_at)),
            pad_label("updated_at", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.updated_at)),
            pad_label("nfts_held", LABEL_LENGTH),
            color.apply_to(self.nfts_held),
            pad_label("vault_balance", LABEL_LENGTH),
            color.apply_to(self.vault_balance),
            pad_label("locked_at", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.locked_at)),
            pad_label("locked_until", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.locked_until)),
            pad_label("taker", LABEL_LENGTH),
            color.apply_to(option_formatter(&self.taker)),
            pad_label("collateral_returned", LABEL_LENGTH),
            color.apply_to(self.collateral_returned),
            pad_label("last_exercised_at", LABEL_LENGTH),
            color.apply_to(format_timestamp(self.last_exercised_at)),
            pad_label("exercise_count", LABEL_LENGTH),
            color.apply_to(self.exercise_count),
            pad_label("accumulated_profit", LABEL_LENGTH),
            color.apply_to(self.accumulated_profit),
            pad_label("taker_withdrawn_nfts", LABEL_LENGTH),
            color.apply_to(self.taker_withdrawn_nfts),
            pad_label("taker_withdrawn_funds", LABEL_LENGTH),
            color.apply_to(self.taker_withdrawn_funds),
            pad_label("early_close", LABEL_LENGTH),
            color.apply_to(self.early_close),
            pad_label("reserved0", LABEL_LENGTH),
            color.apply_to(if self.reserved0.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            }),
            pad_label("reserved1", LABEL_LENGTH),
            color.apply_to(if self.reserved1.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            }),
            pad_label("reserved2", LABEL_LENGTH),
            color.apply_to(if self.reserved2.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            })
        )
    }
}

impl CustomFormat for OrderNftReceipt {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}",
            color.apply_to("OrderNftReceipt---------------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("bump", LABEL_LENGTH),
            color.apply_to(self.bump),
            pad_label("asset", LABEL_LENGTH),
            color.apply_to(self.asset),
            pad_label("order_state", LABEL_LENGTH),
            color.apply_to(self.order_state)
        )
    }
}
