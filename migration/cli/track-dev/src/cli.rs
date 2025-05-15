use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod track;

/// The `mtma-track-dev` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaTrackDev {
	#[clap(subcommand)]
	command: Option<MtmaTrackDevSubcommand>,
}

/// The subcommands of the `mtma-track-dev` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaTrackDevSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Track the state of the node.
	#[clap(subcommand)]
	Track(track::or_file::Track),
}

/// Implement the `From` trait for `MtmaTrackDev` to convert it into a `MtmaTrackDevSubcommand`.
impl From<MtmaTrackDev> for MtmaTrackDevSubcommand {
	fn from(client: MtmaTrackDev) -> Self {
		client.command.unwrap_or(MtmaTrackDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaTrackDev` CLI.
impl MtmaTrackDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaTrackDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaTrackDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaTrackDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaTrackDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaTrackDev>().await?;
			}
			MtmaTrackDevSubcommand::Track(track) => track.execute().await?,
		}
		Ok(())
	}
}
