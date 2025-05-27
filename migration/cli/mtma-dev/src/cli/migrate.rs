use clap::Parser;
use mtma_migrate::cli::MtmaMigrateSubcommand;
use mtma_migrate_dev::cli::MtmaMigrateDevSubcommand;

/// Migrates the node and the chain.
#[derive(Parser)]
#[clap(help_expected = true)]
pub enum Migrate {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(MtmaMigrateDevSubcommand),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(MtmaMigrateSubcommand),
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Migrate::Dev(dev) => dev.execute().await,
			Migrate::Prod(prod) => prod.execute().await,
		}
	}
}
