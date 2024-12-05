use crate::args::{AnchorDiscriminatorArgs, AnchorDiscriminatorKind};

use anyhow::Result;
use sha2::{Digest, Sha256};

pub fn handle_anchor_discriminator(args: AnchorDiscriminatorArgs) -> Result<()> {
    let prefix = match args.kind {
        AnchorDiscriminatorKind::Account => "account",
        AnchorDiscriminatorKind::Instruction => "global",
    };

    let mut hasher = Sha256::new();
    hasher.update(format!("{prefix}:{}", args.name));
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);

    println!(
        "Discriminator (bytes):   [{}]",
        discriminator
            .iter()
            .map(|b| format!("{}", b))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("Discriminator (hex)  :   0x{}", hex::encode(discriminator));

    Ok(())
}
