use console::Style;
use solana_sdk::{account::Account, native_token::lamports_to_sol};

use crate::formatting::pad_label;

use super::CustomFormat;

const LABEL_LENGTH: usize = 15;

impl CustomFormat for Account {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}",
            color.apply_to("Wallet---------"),
            pad_label("lamports", LABEL_LENGTH),
            color.apply_to(self.lamports),
            pad_label("SOL", LABEL_LENGTH),
            color.apply_to(lamports_to_sol(self.lamports)),
        )
    }
}
