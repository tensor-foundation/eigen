use std::str::FromStr;

use super::*;

#[derive(Subcommand)]
pub enum DeriveSubcommands {
    AnchorDisc(AnchorDiscriminatorArgs),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnchorDiscriminatorKind {
    Account,
    Instruction,
}

impl FromStr for AnchorDiscriminatorKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "account" | "acc" | "a" => Self::Account,
            "instruction" | "ix" | "i" => Self::Instruction,
            _ => return Err(format!("Invalid discriminator kind: {}", s)),
        })
    }
}

#[derive(ClapArgs)]
pub struct AnchorDiscriminatorArgs {
    pub kind: AnchorDiscriminatorKind,
    pub name: String,
}
