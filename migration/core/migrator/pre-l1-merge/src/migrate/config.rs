use crate::Migrate;
use anyhow::Context;
use aptos_framework_pre_l1_merge_release::maptos_framework_release_util::LocalAccountReleaseSigner;
use aptos_framework_pre_l1_merge_release::maptos_framework_release_util::OverrideAccountAddressReleaseSigner;
use clap::Parser;
use movement_core::movement::{Celestia, Eth};
use movement_core::Config as MovementCoreConfig;
use movement_signer::key::TryFromCanonicalString;
use movement_signer_loader::identifiers::{local::Local, SignerIdentifier};
use mtma_node_null_core::Config as MtmaNodeNullConfig;
use mtma_types::movement::aptos_sdk::types::{
	account_address::AccountAddress, account_config::aptos_test_root_address, LocalAccount,
};
use mtma_types::movement::movement_config::Config as MovementConfig;
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
	/// The movement config.
	#[clap(flatten)]
	movement_core: MovementCoreConfig,
	/// The account address override for the release signer.
	#[clap(long)]
	pub account_address: Option<AccountAddress>,
}

impl Default for Config {
	fn default() -> Self {
		let mut movement_core = MovementCoreConfig {
			movement_config_string: None,
			setup: false,
			celestia: Celestia::Local,
			eth: Eth::Local,
			biarritz_rc1_to_pre_l1_merge: false,
			ping_rest_api: false,
			ping_faucet: false,
		};

		// Create a default MovementConfig
		let movement_config = MovementConfig::default();

		// Hard-code the core resources address
		let account_address = Some(aptos_test_root_address());

		// Set the movement config
		movement_core.set_movement_config(movement_config);

		Self { mtma_node_null: MtmaNodeNullConfig::default(), movement_core, account_address }
	}
}

impl Config {
	/// Builds the release signer from the config.
	pub fn build_release_signer(&self) -> Result<LocalAccountReleaseSigner, MigrateConfigError> {
		// load the signer
		let raw_private_key = self
			.movement_core
			.try_movement_signer_identifier()
			.map_err(|e| MigrateConfigError::Build(e.into()))?
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
	pub fn build(
		&self,
	) -> Result<
		Migrate<OverrideAccountAddressReleaseSigner<LocalAccountReleaseSigner>>,
		MigrateConfigError,
	> {
		let mtma_node_null =
			self.mtma_node_null.build().map_err(|e| MigrateConfigError::Build(e.into()))?;
		let local_signer = self.build_release_signer()?;
		let signer = OverrideAccountAddressReleaseSigner::core_resource_account(local_signer);
		Ok(Migrate { release_signer: signer, mtma_node_null })
	}
}
