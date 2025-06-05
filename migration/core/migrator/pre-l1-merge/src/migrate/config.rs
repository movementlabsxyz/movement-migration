use crate::Migrate;
use anyhow::Context;
use aptos_framework_pre_l1_merge_release::maptos_framework_release_util::LocalAccountReleaseSigner;
use clap::Parser;
use movement_signer::key::TryFromCanonicalString;
use movement_signer_loader::identifiers::{local::Local, SignerIdentifier};
use mtma_node_null_core::Config as MtmaNodeNullConfig;
use mtma_types::movement::aptos_sdk::types::{account_address::AccountAddress, LocalAccount};
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
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {
	/// The config for the mtma-node-null.
	#[clap(flatten)]
	mtma_node_null: MtmaNodeNullConfig,
	/// The signer identifier for the release signer.
	#[clap(long)]
	#[arg(value_parser = SignerIdentifier::try_from_canonical_string)]
	signer_identifier: SignerIdentifier,
	/// The account address override for the release signer.
	#[clap(long)]
	account_address: Option<AccountAddress>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			mtma_node_null: MtmaNodeNullConfig::default(),
			signer_identifier: SignerIdentifier::Local(Local {
				private_key_hex_bytes:
					"0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
			}),
			account_address: None,
		}
	}
}

impl Config {
	/// Builds the release signer from the config.
	pub fn build_release_signer(&self) -> Result<LocalAccountReleaseSigner, MigrateConfigError> {
		// load the signer
		let raw_private_key = self
			.signer_identifier
			.try_raw_private_key()
			.context("failed to load raw private key")
			.map_err(|e| MigrateConfigError::Build(e.into()))?;

		// get the raw private key
		let private_key_hex = hex::encode(raw_private_key);
		let root_account = LocalAccount::from_private_key(private_key_hex.as_str(), 0)
			.context("failed to build root account")
			.map_err(|e| MigrateConfigError::Build(e.into()))?; // sequence number 0 is fine because the release API loads sequence numbers

		Ok(LocalAccountReleaseSigner::new(root_account, None))
	}

	/// Builds the [Migrate] struct from the config.
	pub fn build(&self) -> Result<Migrate<LocalAccountReleaseSigner>, MigrateConfigError> {
		let mtma_node_null =
			self.mtma_node_null.build().map_err(|e| MigrateConfigError::Build(e.into()))?;

		Ok(Migrate { release_signer: self.build_release_signer()?, mtma_node_null })
	}
}
