use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `mtma-migrate-node` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaMigrateNode {
	#[clap(subcommand)]
	command: Option<MtmaMigrateNodeSubcommand>,
}

/// The subcommands of the `mtma-migrate-node` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaMigrateNodeSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MtmaMigrateNode` to convert it into a `MtmaMigrateNodeSubcommand`.
impl From<MtmaMigrateNode> for MtmaMigrateNodeSubcommand {
	fn from(client: MtmaMigrateNode) -> Self {
		client
			.command
			.unwrap_or(MtmaMigrateNodeSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaMigrateNode` CLI.
impl MtmaMigrateNode {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaMigrateNodeSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaMigrateNodeSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaMigrateNodeSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaMigrateNodeSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaMigrateNode>().await?;
			}
			MtmaMigrateNodeSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
