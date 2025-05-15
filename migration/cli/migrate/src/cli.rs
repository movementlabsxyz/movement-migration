use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `mtma-migrate` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrate {
	#[clap(subcommand)]
	command: Option<MtmaMigrateSubcommand>,
}

/// The subcommands of the `mtma-migrate` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MtmaMigrate` to convert it into a `MtmaMigrateSubcommand`.
impl From<MtmaMigrate> for MtmaMigrateSubcommand {
	fn from(client: MtmaMigrate) -> Self {
		client.command.unwrap_or(MtmaMigrateSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrate` CLI.
impl MtmaMigrate {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrate>().await?;
			}
			MtmaMigrateSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
