use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;
pub mod run;

/// The `movement-to-aptos` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Movement {
	#[clap(subcommand)]
	command: Option<MovementSubcommand>,
}

/// The subcommands of the `movement-to-aptos` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MovementSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Run the movement.
	#[clap(subcommand)]
	Run(run::or_file::Run),
}

/// Implement the `From` trait for `Movement` to convert it into a `MovementSubcommand`.
impl From<Movement> for MovementSubcommand {
	fn from(client: Movement) -> Self {
		client.command.unwrap_or(MovementSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `Movement` CLI.
impl Movement {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MovementSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MovementSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MovementSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MovementSubcommand::Markdown(markdown) => {
				markdown.execute::<Movement>().await?;
			}
			MovementSubcommand::Run(run) => run.execute().await?,
		}
		Ok(())
	}
}
