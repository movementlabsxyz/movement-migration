use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod run;

/// The `movement-to-aptos` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MovementAptos {
	#[clap(subcommand)]
	command: Option<MovementAptosSubcommand>,
}

/// The subcommands of the `movement-to-aptos` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MovementAptosSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Run the movement.
	#[clap(subcommand)]
	Run(run::or_file::Run),
}

/// Implement the `From` trait for `MovementAptos` to convert it into a `MovementAptosSubcommand`.
impl From<MovementAptos> for MovementAptosSubcommand {
	fn from(client: MovementAptos) -> Self {
		client.command.unwrap_or(MovementAptosSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MovementAptos` CLI.
impl MovementAptos {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MovementAptosSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MovementAptosSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MovementAptosSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MovementAptosSubcommand::Markdown(markdown) => {
				markdown.execute::<MovementAptos>().await?;
			}
			MovementAptosSubcommand::Run(run) => run.execute().await?,
		}
		Ok(())
	}
}
