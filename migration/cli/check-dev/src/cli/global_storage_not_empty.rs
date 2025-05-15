use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Checks Global Storage Not Empty criterion
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct GlobalStorageNotEmptyCheck {}

impl GlobalStorageNotEmptyCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::GlobalStorageNotEmptyCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
