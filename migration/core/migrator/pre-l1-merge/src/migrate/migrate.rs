use aptos_config::config::StorageDirPaths;
use aptos_db::AptosDB;
use aptos_framework_pre_l1_merge_release::{
	cached::full::feature_upgrade::PreL1Merge,
	maptos_framework_release_util::{Release, ReleaseSigner},
};
use aptos_storage_interface::DbReaderWriter;
use mtma_migrator_types::migration::{MigrationError, Migrationish};
use mtma_migrator_types::migrator::{
	movement_aptos_migrator::MovementAptosMigrator, movement_migrator::MovementMigrator,
};

use anyhow::Context;
use std::fmt::Debug;
use tracing::info;

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
}

impl<S> Migrate<S>
where
	S: ReleaseSigner + Debug + Clone + Send + Sync + 'static,
{
	pub fn new(release_signer: S) -> Self {
		Self { release_signer }
	}

	pub fn release_signer(&self) -> &S {
		&self.release_signer
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
			.map_err(|e| MigrationError::Internal(e.into()))?;

		todo!()
	}
}
