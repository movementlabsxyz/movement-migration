use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

pub mod all;
pub mod all_of;
pub mod global_storage_includes;
pub mod global_storage_injective;
pub mod global_storage_not_empty;

/// The `mtma-check` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaCheck {
	#[clap(subcommand)]
	command: Option<MtmaCheckSubcommand>,
}

/// The subcommands of the `mtma-check` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaCheckSubcommand {
	/// Generates markdown for the CLI.
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Check all criteria.
	#[clap(subcommand)]
	All(all::or_file::AllCheck),
	/// Check all of the specified criteria.
	#[clap(subcommand)]
	AllOf(all_of::or_file::AllOfCheck),
	/// Check the Global Storage Includes criterion.
	#[clap(subcommand)]
	GlobalStorageIncludes(global_storage_includes::or_file::GlobalStorageIncludesCheck),
	/// Check the Global Storage Injective criterion.
	#[clap(subcommand)]
	GlobalStorageInjective(global_storage_injective::or_file::GlobalStorageInjectiveCheck),
	/// Check the Global Storage Not Empty criterion.
	#[clap(subcommand)]
	GlobalStorageNotEmpty(global_storage_not_empty::or_file::GlobalStorageNotEmptyCheck),
}

/// Implement the `From` trait for `MtmaCheck` to convert it into a `MtmaCheckSubcommand`.
impl From<MtmaCheck> for MtmaCheckSubcommand {
	fn from(client: MtmaCheck) -> Self {
		client.command.unwrap_or(MtmaCheckSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaCheck` CLI.
impl MtmaCheck {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaCheckSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaCheckSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaCheckSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaCheckSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaCheck>().await?;
			}
			MtmaCheckSubcommand::All(check) => check.execute().await?,
			MtmaCheckSubcommand::AllOf(check) => check.execute().await?,
			MtmaCheckSubcommand::GlobalStorageIncludes(check) => check.execute().await?,
			MtmaCheckSubcommand::GlobalStorageInjective(check) => check.execute().await?,
			MtmaCheckSubcommand::GlobalStorageNotEmpty(check) => check.execute().await?,
		}
		Ok(())
	}
}
