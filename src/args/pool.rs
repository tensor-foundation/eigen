use super::*;

#[derive(Subcommand)]
pub enum PoolSubcommands {
    Create(PoolCreateArgs),
}

#[derive(ClapArgs)]
pub struct PoolCreateArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Whitelist public key.
    pub whitelist: Pubkey,
}
