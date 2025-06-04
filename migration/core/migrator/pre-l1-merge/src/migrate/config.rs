use crate::Migrate;
use aptos_framework_pre_l1_merge_release::maptos_framework_release_util::ReleaseSigner;
use clap::Parser;
use mtma_node_null_core::Config as MtmaNodeNullConfig;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
	/// The config for the mtma-node-null.
	#[clap(flatten)]
	mtma_node_null: MtmaNodeNullConfig,
}

impl Config {
	/// Builds the release signer from the config.
	pub fn build_release_signer<S>(&self) -> Result<S, MigrateConfigError>
	where
		S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
	{
		todo!()
	}

	/// Builds the [Migrate] struct from the config.
	pub fn build<S>(&self) -> Result<Migrate<S>, MigrateConfigError>
	where
		S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
	{
		let mtma_node_null =
			self.mtma_node_null.build().map_err(|e| MigrateConfigError::Build(e.into()))?;

		Ok(Migrate { release_signer: self.build_release_signer()?, mtma_node_null })
	}
}
