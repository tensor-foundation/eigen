use super::*;

#[derive(Subcommand)]
pub enum PoolSubcommands {
    Create(PoolCreateArgs),
    Edit(PoolEditArgs),
}

#[derive(ClapArgs)]
pub struct PoolCreateArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Whitelist public key.
    pub whitelist: Pubkey,

    /// Path to the pool config file.
    pub pool_config_path: PathBuf,
}

#[derive(ClapArgs)]
pub struct PoolEditArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Pool public key.
    pub pool: Pubkey,

    /// Path to the edit pool config file.
    pub edit_pool_config_path: PathBuf,
}
