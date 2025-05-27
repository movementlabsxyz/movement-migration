use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;
pub mod migrate_checked;

/// The `mtma-migrate-node-dev` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrateNodeDev {
	#[clap(subcommand)]
	command: Option<MtmaMigrateNodeDevSubcommand>,
}

/// The subcommands of the `mtma-migrate-node-dev` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateNodeDevSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::Migrate),
	/// Migrate from Movement to MovementAptos with checks.
	#[clap(subcommand)]
	MigrateChecked(migrate_checked::or_file::MigrateChecked),
}

/// Implement the `From` trait for `MtmaMigrateNodeDev` to convert it into a `MtmaMigrateNodeDevSubcommand`.
impl From<MtmaMigrateNodeDev> for MtmaMigrateNodeDevSubcommand {
	fn from(client: MtmaMigrateNodeDev) -> Self {
		client
			.command
			.unwrap_or(MtmaMigrateNodeDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrateNodeDev` CLI.
impl MtmaMigrateNodeDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateNodeDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateNodeDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateNodeDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateNodeDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrateNodeDev>().await?;
			}
			MtmaMigrateNodeDevSubcommand::Migrate(migrate) => migrate.execute().await?,
			MtmaMigrateNodeDevSubcommand::MigrateChecked(migrate_checked) => {
				migrate_checked.execute().await?
			}
		}
		Ok(())
	}
}
