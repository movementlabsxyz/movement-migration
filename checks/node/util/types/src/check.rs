use crate::criterion::{Criterionish, MovementNode};
use crate::prelude::Prelude;
use mtma_node_types::migration::Migrationish;

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
	movement_executor: &mut MovementNode,
	prelude: &Prelude,
	migration: &impl Migrationish,
	criteria: Vec<Box<dyn Criterionish + Send + Sync>>,
) -> Result<(), CheckError> {
	// Run the prelude
	prelude
		.run(movement_executor)
		.await
		.map_err(|e| CheckError::Prelude(e.into()))?;

	// Run the migration
	let movement_aptos_executor = migration
		.migrate(movement_executor)
		.await
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	for criterion in criteria {
		criterion
			.satisfies(&movement_executor, &movement_aptos_executor)
			.map_err(|e| CheckError::Criteria(e.into()))?;
	}

	Ok(())
}
