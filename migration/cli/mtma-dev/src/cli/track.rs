use clap::Parser;
use mtma_track::cli::MtmaTrackSubcommand;
use mtma_track_dev::cli::MtmaTrackDevSubcommand;

/// Tracks the node and the chain.
#[derive(Parser)]
#[clap(help_expected = true)]
pub enum Track {
	/// Runs the null migration.
	#[clap(subcommand)]
	Dev(MtmaTrackDevSubcommand),
	/// Runs the replay migration.
	#[clap(subcommand)]
	Prod(MtmaTrackSubcommand),
}

impl Track {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Track::Dev(dev) => dev.execute().await,
			Track::Prod(prod) => prod.execute().await,
		}
	}
}
