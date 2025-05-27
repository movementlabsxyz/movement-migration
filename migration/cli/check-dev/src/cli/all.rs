use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Checks all supported criteria.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct AllCheck {}

impl AllCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::AllCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
