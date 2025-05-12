use crate::Migrate;
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum MigrateConfigError {
	#[error("failed to build from config: {0}")]
	Build(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The config for the framework migration.
///
/// All fields should be easily statically encodable to a CLI argument.
/// This is the frontend for the core API.
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {
	/// The known release you are migrating from, eg elsa or biarritz-rc1.
	#[clap(long)]
	pub from: String,
	/// The known release you are migrating to, eg biarritz-rc1 or pre-l1-merge.
	#[clap(long)]
	pub to: String,
	/// The canonical string for the Maptos signer used in the migration
	#[clap(long)]
	pub maptos_signer: String,
	/// The canonical string for the DA signer used in the migration
	#[clap(long)]
	pub da_signer: String,
	/// The canonical string for the MCR signer used in the migration
	#[clap(long)]
	pub mcr_signer: String,
	/// Movement configuration arguments
	#[clap(long)]
	pub movement_args: String,
}

impl Config {
	/// Builds the [Framework] struct from the config.
	pub fn build(&self) -> Result<Migrate, MigrateConfigError> {
		Ok(Migrate { config: self.clone() })
	}
}
