use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `mtma-migrate-dev` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrateDev {
	#[clap(subcommand)]
	command: Option<MtmaMigrateDevSubcommand>,
}

/// The subcommands of the `mtma-migrate-dev` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateDevSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MtmaMigrateDev` to convert it into a `MtmaMigrateDevSubcommand`.
impl From<MtmaMigrateDev> for MtmaMigrateDevSubcommand {
	fn from(client: MtmaMigrateDev) -> Self {
		client
			.command
			.unwrap_or(MtmaMigrateDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrateDev` CLI.
impl MtmaMigrateDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrateDev>().await?;
			}
			MtmaMigrateDevSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
