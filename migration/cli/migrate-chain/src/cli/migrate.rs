use clap::Parser;
use mtma_core::Config;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Migrates the chain.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Migrate {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let migrate = self.config.build()?;
		migrate.run().await?; // we unwrap the error as an easy way to do marshalling from [MigrateError] to [anyhow::Error]
		Ok(())
	}
}

impl or_file::Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
