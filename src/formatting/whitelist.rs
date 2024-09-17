use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

use super::CustomFormat;

impl CustomFormat for Whitelist {
    fn custom_format(&self) -> String {
        let voc_str = self
            .voc
            .map_or("None".to_string(), |pubkey| pubkey.to_string());
        let fvc_str = self
            .fvc
            .map_or("None".to_string(), |pubkey| pubkey.to_string());

        format!(
            "Whitelist {{
    discriminator: {:?},
    version: {},
    bump: {},
    verified: {},
    root_hash: {},
    uuid: {},
    name: {},
    frozen: {},
    voc: {},
    fvc: {},
    reserved: {}
}}",
            hex::encode(self.discriminator),
            self.version,
            self.bump,
            self.verified,
            hex::encode(self.root_hash),
            String::from_utf8_lossy(&self.uuid),
            String::from_utf8_lossy(&self.name),
            self.frozen,
            voc_str,
            fvc_str,
            if self.reserved.iter().all(|&x| x == 0) {
                "[all zeros]".to_string()
            } else {
                format!("{:?}", &self.reserved[..])
            }
        )
    }
}

impl CustomFormat for WhitelistV2 {
    fn custom_format(&self) -> String {
        format!(
            "WhitelistV2 {{
    discriminator: {:?},
    version: {},
    bump: {},
    uuid: {},
    state: {:?},
    update_authority: {},
    namespace: {},
    freeze_authority: {},
    conditions: {:?}
}}",
            hex::encode(self.discriminator),
            self.version,
            self.bump,
            hex::encode(self.uuid),
            self.state,
            self.update_authority,
            self.namespace,
            self.freeze_authority,
            self.conditions
        )
    }
}

impl CustomFormat for MintProof {
    fn custom_format(&self) -> String {
        format!(
            "MintProof {{
    discriminator: {:?},
    proof_len: {},
    proof: {}
}}",
            hex::encode(self.discriminator),
            self.proof_len,
            if self.proof_len == 0 {
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
            }
        )
    }
}

impl CustomFormat for MintProofV2 {
    fn custom_format(&self) -> String {
        format!(
            "MintProofV2 {{
    discriminator: {:?},
    proof_len: {},
    proof: {},
    creation_slot: {},
    payer: {}
}}",
            hex::encode(self.discriminator),
            self.proof_len,
            if self.proof_len == 0 {
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
            },
            self.creation_slot,
            self.payer
        )
    }
}
