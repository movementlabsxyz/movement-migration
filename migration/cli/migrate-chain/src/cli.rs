use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `mtma-migrate-chain` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrateChain {
	#[clap(subcommand)]
	command: Option<MtmaMigrateChainSubcommand>,
}

/// The subcommands of the `mtma-migrate-chain` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateChainSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MtmaMigrateChain` to convert it into a `MtmaMigrateChainSubcommand`.
impl From<MtmaMigrateChain> for MtmaMigrateChainSubcommand {
	fn from(client: MtmaMigrateChain) -> Self {
		client
			.command
			.unwrap_or(MtmaMigrateChainSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrateChain` CLI.
impl MtmaMigrateChain {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateChainSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateChainSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateChainSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateChainSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrateChain>().await?;
			}
			MtmaMigrateChainSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
