use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Runs the null migration.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Null {}

impl Null {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		unimplemented!()
	}
}

impl or_file::Null {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
