use super::*;

#[derive(Subcommand)]
pub enum WhitelistSubcommands {
    Compare(WhitelistCompareArgs),
    Create(WhitelistCreateArgs),
}

#[derive(ClapArgs)]
pub struct WhitelistCompareArgs {
    #[clap(flatten)]
    pub read_options: ReadOptions,

    /// List file path.
    pub list: Option<PathBuf>,

    /// Verbose output.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ClapArgs)]
pub struct WhitelistCreateArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,

    /// Namespace path.
    pub namespace_path: Option<PathBuf>,
}
