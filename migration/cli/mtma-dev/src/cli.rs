use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod check;
pub mod migrate;
pub mod track;

/// The `movement-to-aptos` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaDev {
	#[clap(subcommand)]
	command: Option<MtmaDevSubcommand>,
}

/// The subcommands of the `movement-to-aptos` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaDevSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Check the state for given migration criteria.
	#[clap(subcommand)]
	Check(check::Check),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::Migrate),
	/// Track the state of the node for real-time comparison.
	#[clap(subcommand)]
	Track(track::Track),
}

/// Implement the `From` trait for `MtmaDev` to convert it into a `MtmaDevSubcommand`.
impl From<MtmaDev> for MtmaDevSubcommand {
	fn from(client: MtmaDev) -> Self {
		client.command.unwrap_or(MtmaDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaDev` CLI.
impl MtmaDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaDev>().await?;
			}
			MtmaDevSubcommand::Check(check) => check.execute().await?,
			MtmaDevSubcommand::Migrate(migrate) => migrate.execute().await?,
			MtmaDevSubcommand::Track(track) => track.execute().await?,
		}
		Ok(())
	}
}
