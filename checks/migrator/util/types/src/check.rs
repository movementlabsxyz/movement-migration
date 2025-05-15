use crate::criterion::{Criterionish, MovementMigrator};
use mtma_migrator_types::migration::Migrationish;
use mtma_node_test_types::prelude::Prelude;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
	#[error("failed to run prelude: {0}")]
	Prelude(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to run migration: {0}")]
	Migration(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to satisfy criteria: {0}")]
	Criteria(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Runs a migration where prelude is executed, the migration is run, and then the criteria are checked.
pub async fn checked_migration(
	movement_migrator: &mut MovementMigrator,
	prelude: &Prelude,
	migration: &impl Migrationish,
	criteria: Vec<Box<dyn Criterionish + Send + Sync>>,
) -> Result<(), CheckError> {
	// Get the executor
	let mut movement_executor =
		movement_migrator.executor().await.map_err(|e| CheckError::Internal(e.into()))?;

	// Run the prelude
	prelude
		.run(&mut movement_executor)
		.await
		.map_err(|e| CheckError::Prelude(e.into()))?;

	// Run the migration
	let movement_aptos_migrator = migration
		.migrate(movement_migrator)
		.await
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	for criterion in criteria {
		criterion
			.satisfies(&movement_migrator, &movement_aptos_migrator)
			.map_err(|e| CheckError::Criteria(e.into()))?;
	}

	Ok(())
}
