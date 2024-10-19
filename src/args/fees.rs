use super::*;

#[derive(Subcommand)]
pub enum FeesSubcommands {
    Balances(FeesBalancesArgs),
    Fund(FeesFundArgs),
    Shards,
}

#[derive(ClapArgs)]
pub struct FeesBalancesArgs {
    #[clap(flatten)]
    pub read_options: ReadOptions,
}

#[derive(ClapArgs)]
pub struct FeesFundArgs {
    #[clap(flatten)]
    pub write_options: WriteOptions,
}
