use chrono::DateTime;
use tensor_amm::{
    accounts::{NftDepositReceipt, Pool},
    types::{PoolConfig, PoolStats},
};

use super::CustomFormat;

impl CustomFormat for Pool {
    fn custom_format(&self) -> String {
        fn format_timestamp(timestamp: i64) -> String {
            DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_default()
                .to_rfc3339()
        }

        format!(
            "Pool {{
    discriminator: {:?},
    version: {},
    bump: {:?},
    pool_id: {},
    created_at: {},
    updated_at: {},
    expiry: {},
    owner: {},
    whitelist: {},
    rent_payer: {},
    currency: {:?},
    amount: {},
    price_offset: {},
    nfts_held: {},
    stats: {:?},
    shared_escrow: {:?},
    cosigner: {:?},
    maker_broker: {:?},
    max_taker_sell_count: {},
    config: {:?},
    reserved: {}
}}",
            hex::encode(self.discriminator),
            self.version,
            self.bump[0],
            String::from_utf8_lossy(&self.pool_id),
            format_timestamp(self.created_at),
            format_timestamp(self.updated_at),
            format_timestamp(self.expiry),
            self.owner,
            self.whitelist,
            self.rent_payer,
            self.currency,
            self.amount,
            self.price_offset,
            self.nfts_held,
            self.stats,
            self.shared_escrow,
            self.cosigner,
            self.maker_broker,
            self.max_taker_sell_count,
            self.config,
            if self.reserved.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                format!("{:?}", &self.reserved[..])
            }
        )
    }
}

impl CustomFormat for NftDepositReceipt {
    fn custom_format(&self) -> String {
        format!(
            "NftDepositReceipt:\n\
             - Bump: {}\n\
             - Mint: {}\n\
             - Pool: {}",
            self.bump, self.mint, self.pool
        )
    }
}

impl CustomFormat for PoolConfig {
    fn custom_format(&self) -> String {
        format!(
            "PoolConfig:\n\
             - Pool Type: {:?}\n\
             - Curve Type: {:?}\n\
             - Starting Price: {}\n\
             - Delta: {}\n\
             - MM Compound Fees: {}\n\
             - MM Fee BPS: {}",
            self.pool_type,
            self.curve_type,
            self.starting_price,
            self.delta,
            self.mm_compound_fees,
            self.mm_fee_bps
                .to_option()
                .map_or("None".to_string(), |bps| bps.to_string())
        )
    }
}

impl CustomFormat for PoolStats {
    fn custom_format(&self) -> String {
        format!(
            "PoolStats:\n\
             - Taker Sell Count: {}\n\
             - Taker Buy Count: {}\n\
             - Accumulated MM Profit: {}",
            self.taker_sell_count, self.taker_buy_count, self.accumulated_mm_profit
        )
    }
}
