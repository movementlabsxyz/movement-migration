pub mod core;
pub mod select;

use clap::Parser;

/// Migrates the node.
#[derive(Parser, Debug)]
#[clap(help_expected = true)]
pub enum Migrate {
	/// Core migration over the node.
	Core(core::Core),
	/// Select migration over the node.
	Select(select::select::Select),
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Migrate::Core(core) => core.execute().await,
			Migrate::Select(select) => select.execute().await,
		}
	}
}
