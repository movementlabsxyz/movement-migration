use crate::Migrate;
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum MigrateConfigError {
	#[error("failed to build from config: {0}")]
	Build(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The config for the migration.
///
/// All fields should be easily statically encodable to a CLI argument.
/// This is the frontend for the core API.
#[derive(Parser, Debug, Default, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {}

impl Config {
	/// Builds the [Migrate] struct from the config.
	pub fn build(&self) -> Result<Migrate, MigrateConfigError> {
		Ok(Migrate {})
	}
}
