use clap::Parser;
use mtma_check::cli::MtmaCheckSubcommand;
use mtma_check_dev::cli::MtmaCheckDevSubcommand;

/// Checks the node and the chain.
#[derive(Parser)]
#[clap(help_expected = true)]
pub enum Check {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(MtmaCheckDevSubcommand),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(MtmaCheckSubcommand),
}

impl Check {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Check::Dev(dev) => dev.execute().await,
			Check::Prod(prod) => prod.execute().await,
		}
	}
}
