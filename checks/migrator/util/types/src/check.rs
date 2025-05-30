use crate::criterion::{Criterionish, MovementMigrator};
use anyhow::Context;
use mtma_migrator_types::migration::Migrationish;
use mtma_node_test_types::prelude::Prelude;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
	#[error("checked migration encountered an error while running prelude: {0}")]
	Prelude(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("checked migration encountered an error while running migration: {0}")]
	Migration(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("checked migration encountered an error while satisfying criteria: {0}")]
	Criteria(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("checked migration encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Runs a migration where prelude is executed, the migration is run, and then the criteria are checked.
pub async fn checked_migration<T: Criterionish + Send + Sync>(
	movement_migrator: &mut MovementMigrator,
	prelude: &Prelude,
	migration: &impl Migrationish,
	criteria: Vec<T>,
) -> Result<(), CheckError> {
	// Get the executor
	let mut movement_executor = movement_migrator
		.node()
		.await
		.context("failed to get movement node")
		.map_err(|e| CheckError::Internal(e.into()))?;

	// Run the prelude
	prelude
		.run(&mut movement_executor)
		.await
		.context("failed to run prelude")
		.map_err(|e| CheckError::Prelude(e.into()))?;

	// Run the migration
	let movement_aptos_migrator = migration
		.migrate(movement_migrator)
		.await
		.context("failed to run migration")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	for criterion in criteria {
		criterion
			.satisfies(&movement_migrator, &movement_aptos_migrator)
			.await
			.context("failed to satisfy criterion")
			.map_err(|e| CheckError::Criteria(e.into()))?;
	}

	Ok(())
}
