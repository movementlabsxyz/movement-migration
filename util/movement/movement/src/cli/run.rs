use clap::Parser;
use movement_core::Config;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Run {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
}

impl Run {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let run = self.config.build()?;
		run.run().await?; // we unwrap the error as an easy way to do marshalling from [MigrateError] to [anyhow::Error]
		Ok(())
	}
}

impl or_file::Run {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
