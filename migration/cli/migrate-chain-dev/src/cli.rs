use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `mtma-migrate-chain-dev` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrateChainDev {
	#[clap(subcommand)]
	command: Option<MtmaMigrateChainDevSubcommand>,
}

/// The subcommands of the `mtma-migrate-chain-dev` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateChainDevSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MtmaMigrateChainDev` to convert it into a `MtmaMigrateChainDevSubcommand`.
impl From<MtmaMigrateChainDev> for MtmaMigrateChainDevSubcommand {
	fn from(client: MtmaMigrateChainDev) -> Self {
		client
			.command
			.unwrap_or(MtmaMigrateChainDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrateChainDev` CLI.
impl MtmaMigrateChainDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateChainDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateChainDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateChainDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateChainDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrateChainDev>().await?;
			}
			MtmaMigrateChainDevSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
