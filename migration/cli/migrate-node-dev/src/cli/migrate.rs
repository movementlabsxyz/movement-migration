use clap::Parser;
pub mod null;
pub mod replay;

/// Migrates the node.
#[derive(Parser, Debug, Clone)]
#[clap(help_expected = true)]
pub enum Migrate {
	/// Runs the null migration.
	#[clap(subcommand)]
	Null(null::or_file::Null),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Replay(replay::or_file::Replay),
}

impl Migrate {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Migrate::Null(null) => null.execute().await,
			Migrate::Replay(replay) => replay.execute().await,
		}
	}
}
