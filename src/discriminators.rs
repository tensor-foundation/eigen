use anyhow::{anyhow, Result};
use borsh::BorshDeserialize;
use sha2::{Digest, Sha256};
use tensor_amm::accounts::{NftDepositReceipt, Pool};
use tensor_marketplace::accounts::{BidState, ListState};
use tensor_whitelist::accounts::{MintProof, MintProofV2, Whitelist, WhitelistV2};

pub trait Discriminator {
    fn discriminator() -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "account:{}",
            std::any::type_name::<Self>()
                .split("::")
                .last()
                .expect("No type name found")
        ));
        let result = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&result[..8]);
        discriminator
    }
}

pub fn deserialize_account<T: BorshDeserialize + Discriminator>(mut data: &[u8]) -> Result<T> {
    if data.len() < 8 {
        return Err(anyhow!("Data too short"));
    }

    let discriminator: [u8; 8] = data[..8].try_into()?;
    if discriminator != T::discriminator() {
        return Err(anyhow!("Invalid discriminator for type"));
    }

    T::deserialize(&mut data).map_err(Into::into)
}

impl Discriminator for Pool {}
impl Discriminator for NftDepositReceipt {}

impl Discriminator for Whitelist {}
impl Discriminator for WhitelistV2 {}
impl Discriminator for MintProof {}
impl Discriminator for MintProofV2 {}
impl Discriminator for BidState {}
impl Discriminator for ListState {}
