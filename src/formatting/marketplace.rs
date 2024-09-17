use chrono::DateTime;
use tensor_marketplace::accounts::{BidState, ListState};

use super::CustomFormat;

impl CustomFormat for BidState {
    fn custom_format(&self) -> String {
        fn format_timestamp(timestamp: i64) -> String {
            DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }

        format!(
            "BidState {{
    discriminator: {:?},
    version: {},
    bump: {:?},
    owner: {},
    bid_id: {},
    target: {:?},
    target_id: {},
    field: {:?},
    field_id: {:?},
    quantity: {},
    filled_quantity: {},
    amount: {},
    currency: {:?},
    expiry: {},
    private_taker: {:?},
    maker_broker: {:?},
    margin: {:?},
    updated_at: {},
    cosigner: {:?},
    rent_payer: {:?},
    reserved: {},
    reserved1: {},
    reserved2: {}
}}",
            hex::encode(self.discriminator),
            self.version,
            self.bump[0],
            self.owner,
            self.bid_id,
            self.target,
            self.target_id,
            self.field,
            self.field_id,
            self.quantity,
            self.filled_quantity,
            self.amount,
            self.currency,
            format_timestamp(self.expiry),
            self.private_taker,
            self.maker_broker,
            self.margin,
            format_timestamp(self.updated_at),
            self.cosigner,
            self.rent_payer,
            if self.reserved.iter().all(|&x| x == 0) {
                "[all zeros]"
            } else {
                "[non-zero]"
            },
            if self.reserved1.iter().all(|&x| x == 0) {
                "[all zeros]"
            } else {
                "[non-zero]"
            },
            if self.reserved2.iter().all(|&x| x == 0) {
                "[all zeros]"
            } else {
                "[non-zero]"
            }
        )
    }
}

impl CustomFormat for ListState {
    fn custom_format(&self) -> String {
        fn format_timestamp(timestamp: i64) -> String {
            DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }

        format!(
            "ListState {{
    discriminator: {:?},
    version: {},
    bump: {:?},
    owner: {},
    asset_id: {},
    amount: {},
    currency: {:?},
    expiry: {},
    private_taker: {:?},
    maker_broker: {:?},
    rent_payer: {:?},
    cosigner: {:?},
    reserved1: {}
}}",
            hex::encode(self.discriminator),
            self.version,
            self.bump[0],
            self.owner,
            self.asset_id,
            self.amount,
            self.currency,
            format_timestamp(self.expiry),
            self.private_taker,
            self.maker_broker,
            self.rent_payer,
            self.cosigner,
            if self.reserved1.iter().all(|&x| x == 0) {
                "[all zeros]"
            } else {
                "[non-zero]"
            }
        )
    }
}
