use console::Style;
use tensor_marketplace::accounts::{BidState, ListState};

use crate::formatting::{format_timestamp, option_formatter, pad_label};

use super::CustomFormat;

const LABEL_LENGTH: usize = 20;

impl CustomFormat for BidState {
    fn custom_format(&self) -> String {
        let magenta = Style::new().magenta();

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
{}: {}",
            magenta.apply_to("BidState------------"),
            pad_label("discriminator", LABEL_LENGTH),
            magenta.apply_to(hex::encode(self.discriminator)),
            pad_label("version", LABEL_LENGTH),
            magenta.apply_to(self.version),
            pad_label("bump", LABEL_LENGTH),
            magenta.apply_to(self.bump[0]),
            pad_label("owner", LABEL_LENGTH),
            magenta.apply_to(self.owner),
            pad_label("bid_id", LABEL_LENGTH),
            magenta.apply_to(self.bid_id),
            pad_label("target", LABEL_LENGTH),
            magenta.apply_to(self.target),
            pad_label("target_id", LABEL_LENGTH),
            magenta.apply_to(self.target_id),
            pad_label("field", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.field)),
            pad_label("field_id", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.field_id)),
            pad_label("quantity", LABEL_LENGTH),
            magenta.apply_to(self.quantity),
            pad_label("filled_quantity", LABEL_LENGTH),
            magenta.apply_to(self.filled_quantity),
            pad_label("amount", LABEL_LENGTH),
            magenta.apply_to(self.amount),
            pad_label("currency", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.currency)),
            pad_label("expiry", LABEL_LENGTH),
            magenta.apply_to(format_timestamp(self.expiry)),
            pad_label("private_taker", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.private_taker)),
            pad_label("maker_broker", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.maker_broker)),
            pad_label("margin", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.margin)),
            pad_label("updated_at", LABEL_LENGTH),
            magenta.apply_to(format_timestamp(self.updated_at)),
            pad_label("cosigner", LABEL_LENGTH),
            magenta.apply_to(self.cosigner),
            pad_label("rent_payer", LABEL_LENGTH),
            magenta.apply_to(self.rent_payer),
            pad_label("reserved", LABEL_LENGTH),
            magenta.apply_to(if self.reserved.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            }),
            pad_label("reserved1", LABEL_LENGTH),
            magenta.apply_to(if self.reserved1.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            }),
            pad_label("reserved2", LABEL_LENGTH),
            magenta.apply_to(if self.reserved2.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            })
        )
    }
}

impl CustomFormat for ListState {
    fn custom_format(&self) -> String {
        let magenta = Style::new().magenta();

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
{}: {}",
            magenta.apply_to("ListState-----------"),
            pad_label("discriminator", LABEL_LENGTH),
            magenta.apply_to(hex::encode(self.discriminator)),
            pad_label("version", LABEL_LENGTH),
            magenta.apply_to(self.version),
            pad_label("bump", LABEL_LENGTH),
            magenta.apply_to(self.bump[0]),
            pad_label("owner", LABEL_LENGTH),
            magenta.apply_to(self.owner),
            pad_label("asset_id", LABEL_LENGTH),
            magenta.apply_to(self.asset_id),
            pad_label("amount", LABEL_LENGTH),
            magenta.apply_to(self.amount),
            pad_label("currency", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.currency)),
            pad_label("expiry", LABEL_LENGTH),
            magenta.apply_to(format_timestamp(self.expiry)),
            pad_label("private_taker", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.private_taker)),
            pad_label("maker_broker", LABEL_LENGTH),
            magenta.apply_to(option_formatter(&self.maker_broker)),
            pad_label("rent_payer", LABEL_LENGTH),
            magenta.apply_to(format!("{}", self.rent_payer)),
            pad_label("cosigner", LABEL_LENGTH),
            magenta.apply_to(format!("{}", &self.cosigner)),
            pad_label("reserved1", LABEL_LENGTH),
            magenta.apply_to(if self.reserved1.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                "[non-zero]".to_string()
            })
        )
    }
}
