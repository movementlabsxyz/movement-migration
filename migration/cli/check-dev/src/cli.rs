use clap::{Parser, Subcommand};
use clap_markdown_ext::Markdown;

pub mod all;
pub mod all_of;
pub mod global_storage_includes;
pub mod global_storage_injective;
pub mod global_storage_not_empty;

/// The `mtma-check-dev` CLI.
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct MtmaCheckDev {
	#[clap(subcommand)]
	command: Option<MtmaCheckDevSubcommand>,
}

/// The subcommands of the `mtma-check-dev` CLI.
#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum MtmaCheckDevSubcommand {
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

/// Implement the `From` trait for `MtmaCheckDev` to convert it into a `MtmaCheckDevSubcommand`.
impl From<MtmaCheckDev> for MtmaCheckDevSubcommand {
	fn from(client: MtmaCheckDev) -> Self {
		client.command.unwrap_or(MtmaCheckDevSubcommand::Markdown(Markdown::default()))
	}
}

/// Implement the `MtmaCheckDev` CLI.
impl MtmaCheckDev {
	pub async fn execute(self) -> Result<(), anyhow::Error> {
		let subcommand: MtmaCheckDevSubcommand = self.into();
		subcommand.execute().await
	}
}

/// Implement the `MtmaCheckDevSubcommand` CLI.
/// This is where the actual logic of the CLI is implemented.
impl MtmaCheckDevSubcommand {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			MtmaCheckDevSubcommand::Markdown(markdown) => {
				markdown.execute::<MtmaCheckDev>().await?;
			}
			MtmaCheckDevSubcommand::All(check) => check.execute().await?,
			MtmaCheckDevSubcommand::AllOf(check) => check.execute().await?,
			MtmaCheckDevSubcommand::GlobalStorageIncludes(check) => check.execute().await?,
			MtmaCheckDevSubcommand::GlobalStorageInjective(check) => check.execute().await?,
			MtmaCheckDevSubcommand::GlobalStorageNotEmpty(check) => check.execute().await?,
		}
		Ok(())
	}
}
