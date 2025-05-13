use aptos_framework_pre_l1_merge_release::{
	cached::full::feature_upgrade::PreL1Merge,
	maptos_framework_release_util::{Release, ReleaseSigner},
};
use mtma_migrator_types::migration::{MigrationError, Migrationish};
use mtma_migrator_types::migrator::{
	movement_aptos_migrator::MovementAptosMigrator, movement_migrator::MovementMigrator,
};
use mtma_node_null_core::Migrate as MtmaNodeNullMigrate;

use anyhow::Context;
use std::fmt::Debug;

/// Errors thrown during the migration.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
	#[error("failed to migrate with pre-l1-merge: {0}")]
	Migrate(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The migration struct will be use to run a migration from Movement
#[derive(Debug, Clone)]
pub struct Migrate<S>
where
	S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
{
	pub(crate) release_signer: S,
	pub(crate) mtma_node_null: MtmaNodeNullMigrate,
}

impl<S> Migrate<S>
where
	S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
{
	pub fn new(release_signer: S, mtma_node_null: MtmaNodeNullMigrate) -> Self {
		Self { release_signer, mtma_node_null }
	}

	pub fn release_signer(&self) -> &S {
		&self.release_signer
	}

	pub fn mtma_node_null(&self) -> &MtmaNodeNullMigrate {
		&self.mtma_node_null
	}
}

impl<S> Migrationish for Migrate<S>
where
	S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
{
	async fn migrate(
		&self,
		movement_migrator: &MovementMigrator,
	) -> Result<MovementAptosMigrator, MigrationError> {
		// wait for the rest client to be ready
		let rest_client = movement_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(300))
			.await
			.context("failed to wait for rest client to be ready")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// build the pre-l1-merge
		let pre_l1_merge = PreL1Merge::new();

		// Run the release
		pre_l1_merge
			.release(self.release_signer(), 2_000_000, 100, 60, &rest_client)
			.await
			.context("failed to release with pre-l1-merge")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// migrate the node
		let movement_aptos_node = self
			.mtma_node_null
			.migrate(movement_migrator)
			.await
			.context("failed to migrate node with mtma-node-null")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		let movement_aptos_migrator = movement_aptos_node
			.try_into()
			.context("failed to convert movement aptos node to movement aptos migrator")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// return the movement aptos migrator
		Ok(movement_aptos_migrator)
	}
}
