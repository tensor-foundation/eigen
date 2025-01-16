use super::*;

#[derive(Subcommand)]
pub enum WhitelistSubcommands {
    Compare(WhitelistCompareArgs),
    Create(WhitelistCreateArgs),
    Update(WhitelistUpdateArgs),
}

#[derive(ClapArgs)]
pub struct WhitelistCompareArgs {
    #[clap(flatten)]
    pub read_options: ReadOptions,

    /// List file path.
    #[arg(short, long)]
    pub list: Option<PathBuf>,

    /// Optional namespace to derive whitelist v2 address.
    #[arg(short, long)]
    pub namespace: Option<Pubkey>,

    /// Verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ClapArgs)]
pub struct WhitelistCreateArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Whitelist config path.
    pub whitelist_config_path: PathBuf,

    /// Namespace path.
    pub namespace_path: Option<PathBuf>,
}

#[derive(ClapArgs)]
pub struct WhitelistUpdateArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Whitelist address.
    pub whitelist_address: Pubkey,

    /// New conditions path.
    #[arg(short = 'c', long)]
    pub new_conditions_path: Option<PathBuf>,

    /// New update authority json file path.
    #[arg(short = 'u', long)]
    pub new_update_authority_path: Option<PathBuf>,

    /// New freeze authority.
    #[arg(short = 'f', long)]
    pub new_freeze_authority: Option<Pubkey>,
}
