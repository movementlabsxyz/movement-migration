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
pub struct Config {
	/// Whether to use the migrated genesis.
	pub use_migrated_genesis: bool,
}

impl Config {
	/// Builder API: sets the [use_migrated_genesis] field.
	pub fn use_migrated_genesis(mut self, use_migrated_genesis: bool) -> Self {
		self.use_migrated_genesis = use_migrated_genesis;
		self
	}

	/// Builds the [Migrate] struct from the config.
	pub fn build(&self) -> Result<Migrate, MigrateConfigError> {
		Ok(Migrate { use_migrated_genesis: self.use_migrated_genesis })
	}
}
