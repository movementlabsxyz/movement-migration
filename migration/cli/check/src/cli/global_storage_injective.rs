use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Checks Global Storage Injective criterion
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct GlobalStorageInjectiveCheck {}

impl GlobalStorageInjectiveCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::GlobalStorageInjectiveCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
