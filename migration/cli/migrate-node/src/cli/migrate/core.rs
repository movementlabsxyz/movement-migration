use clap::Parser;
use mtma_core::Config;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Core migration over the node.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Core {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
}

impl Core {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let migrate = self.config.build()?;
		migrate.run().await?; // we unwrap the error as an easy way to do marshalling from [CoreError] to [anyhow::Error]
		Ok(())
	}
}

impl or_file::Core {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
