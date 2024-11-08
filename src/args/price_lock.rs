use super::*;

#[derive(Subcommand)]
pub enum PriceLockSubcommands {
    Find(PriceLockFindArgs),
    Close(PriceLockCloseArgs),
}

#[derive(ClapArgs)]
pub struct PriceLockFindArgs {
    #[clap(flatten)]
    pub read_options: ReadOptions,
}

#[derive(ClapArgs)]
pub struct PriceLockCloseArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,
}
