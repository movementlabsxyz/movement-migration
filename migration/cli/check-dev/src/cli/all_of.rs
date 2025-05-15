use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Checks all of the criteria provided.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct AllOfCheck {}

impl AllOfCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::AllOfCheck {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
