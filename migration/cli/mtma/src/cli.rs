use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod migrate;

/// The `movement-to-aptos` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MovementToMovementAptos {
	#[clap(subcommand)]
	command: Option<MovementToMovementAptosSubcommand>,
}

/// The subcommands of the `movement-to-aptos` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MovementToMovementAptosSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Migrate from Movement to MovementAptos.
	#[clap(subcommand)]
	Migrate(migrate::or_file::Migrate),
}

/// Implement the `From` trait for `MovementToMovementAptos` to convert it into a `MovementToMovementAptosSubcommand`.
impl From<MovementToMovementAptos> for MovementToMovementAptosSubcommand {
	fn from(client: MovementToMovementAptos) -> Self {
		client
			.command
			.unwrap_or(MovementToMovementAptosSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MovementToMovementAptos` CLI.
impl MovementToMovementAptos {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MovementToMovementAptosSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MovementToMovementAptosSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MovementToMovementAptosSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MovementToMovementAptosSubcommand::Markdown(markdown) => {
				markdown.execute::<MovementToMovementAptos>().await?;
			}
			MovementToMovementAptosSubcommand::Migrate(migrate) => migrate.execute().await?,
		}
		Ok(())
	}
}
