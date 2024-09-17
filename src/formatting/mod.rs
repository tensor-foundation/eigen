pub mod amm;
pub mod marketplace;
pub mod whitelist;

pub trait CustomFormat {
    fn custom_format(&self) -> String;
}
