use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod track;

/// The `mtma-track` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaTrack {
	#[clap(subcommand)]
	command: Option<MtmaTrackSubcommand>,
}

/// The subcommands of the `mtma-track` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaTrackSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Track the state of the node.
	#[clap(subcommand)]
	Track(track::or_file::Track),
}

/// Implement the `From` trait for `MtmaTrack` to convert it into a `MtmaTrackSubcommand`.
impl From<MtmaTrack> for MtmaTrackSubcommand {
	fn from(client: MtmaTrack) -> Self {
		client.command.unwrap_or(MtmaTrackSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaTrack` CLI.
impl MtmaTrack {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaTrackSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaTrackSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaTrackSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaTrackSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaTrack>().await?;
			}
			MtmaTrackSubcommand::Track(track) => track.execute().await?,
		}
		Ok(())
	}
}
