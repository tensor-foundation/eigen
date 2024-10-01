use console::Style;

use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

use crate::{commands::ComparisonResult, formatting::pad_label};

use super::CustomFormat;

const LABEL_LENGTH: usize = 20;

impl CustomFormat for Whitelist {
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
{}: {}",
            color.apply_to("Whitelist-----------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("version", LABEL_LENGTH),
            color.apply_to(self.version),
            pad_label("bump", LABEL_LENGTH),
            color.apply_to(self.bump),
            pad_label("verified", LABEL_LENGTH),
            color.apply_to(self.verified),
            pad_label("root_hash", LABEL_LENGTH),
            color.apply_to(hex::encode(self.root_hash)),
            pad_label("uuid", LABEL_LENGTH),
            color.apply_to(String::from_utf8_lossy(&self.uuid)),
            pad_label("name", LABEL_LENGTH),
            color.apply_to(String::from_utf8_lossy(&self.name)),
            pad_label("frozen", LABEL_LENGTH),
            color.apply_to(self.frozen),
            pad_label("voc", LABEL_LENGTH),
            color.apply_to(
                self.voc
                    .map_or("None".to_string(), |pubkey| pubkey.to_string())
            ),
            pad_label("fvc", LABEL_LENGTH),
            color.apply_to(
                self.fvc
                    .map_or("None".to_string(), |pubkey| pubkey.to_string())
            ),
            pad_label("reserved", LABEL_LENGTH),
            color.apply_to(if self.reserved.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                format!("{:?}", &self.reserved[..])
            })
        )
    }
}

impl CustomFormat for WhitelistV2 {
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
{}: {}",
            color.apply_to("WhitelistV2---------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("version", LABEL_LENGTH),
            color.apply_to(self.version),
            pad_label("bump", LABEL_LENGTH),
            color.apply_to(self.bump),
            pad_label("uuid", LABEL_LENGTH),
            color.apply_to(hex::encode(self.uuid)),
            pad_label("state", LABEL_LENGTH),
            color.apply_to(format!("{:?}", self.state)),
            pad_label("update_authority", LABEL_LENGTH),
            color.apply_to(self.update_authority),
            pad_label("namespace", LABEL_LENGTH),
            color.apply_to(self.namespace),
            pad_label("freeze_authority", LABEL_LENGTH),
            color.apply_to(self.freeze_authority),
            pad_label("conditions", LABEL_LENGTH),
            color.apply_to(format!("{:?}", self.conditions))
        )
    }
}

impl CustomFormat for MintProof {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}",
            color.apply_to("MintProof----------------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("proof_len", LABEL_LENGTH),
            color.apply_to(self.proof_len),
            pad_label("proof", LABEL_LENGTH),
            color.apply_to(if self.proof_len == 0 {
                "[empty]".to_string()
            } else {
                format!(
                    "[{} entries]",
                    self.proof[..self.proof_len as usize]
                        .iter()
                        .map(hex::encode)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
        )
    }
}

impl CustomFormat for MintProofV2 {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();

        format!(
            "{}
{}: {}
{}: {}
{}: {}
{}: {}
{}: {}",
            color.apply_to("MintProofV2--------------"),
            pad_label("discriminator", LABEL_LENGTH),
            color.apply_to(hex::encode(self.discriminator)),
            pad_label("proof_len", LABEL_LENGTH),
            color.apply_to(self.proof_len),
            pad_label("proof", LABEL_LENGTH),
            color.apply_to(if self.proof_len == 0 {
                "[empty]".to_string()
            } else {
                format!(
                    "[{} entries]",
                    self.proof[..self.proof_len as usize]
                        .iter()
                        .map(hex::encode)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }),
            pad_label("creation_slot", LABEL_LENGTH),
            color.apply_to(self.creation_slot),
            pad_label("payer", LABEL_LENGTH),
            color.apply_to(self.payer)
        )
    }
}

impl CustomFormat for ComparisonResult {
    fn custom_format(&self) -> String {
        // Use the default text color but set this up for future use.
        let color = Style::new();
        let check = "✅";
        let cross = "X";

        format!(
            "{}
{}: {}
{}: {}
{}: {}",
            color.apply_to("Whitelist Comparison----"),
            pad_label("Whitelist V1", LABEL_LENGTH),
            color.apply_to(self.whitelist_v1),
            pad_label("Whitelist V2", LABEL_LENGTH),
            color.apply_to(self.whitelist_v2),
            pad_label("Whitelists match", LABEL_LENGTH),
            color.apply_to(if self.mismatch.is_none() {
                check
            } else {
                cross
            }),
        )
    }
}
