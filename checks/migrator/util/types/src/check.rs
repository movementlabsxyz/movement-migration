use crate::criterion::{Criterionish, MovementMigrator};
use anyhow::Context;
use mtma_migrator_types::migration::Migrationish;
use mtma_node_test_types::prelude::Prelude;
use tracing::info;

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
	// reset the states of the movement migrator
	info!("Resetting movement migrator states");
	movement_migrator
		.reset_states()
		.await
		.context("failed to reset movement migrator states")
		.map_err(|e| CheckError::Internal(e.into()))?;

	// Get the executor
	info!("Getting movement executor");
	let mut movement_executor = movement_migrator
		.node()
		.await
		.context("failed to get movement node")
		.map_err(|e| CheckError::Internal(e.into()))?;

	// Run the prelude
	info!("Running prelude");
	prelude
		.run(&mut movement_executor)
		.await
		.context("failed to run prelude")
		.map_err(|e| CheckError::Prelude(e.into()))?;

	// Run the migration
	info!("Running migration");
	println!("Running migration");
	let movement_aptos_migrator = migration
		.migrate(movement_migrator)
		.await
		.context("failed to run migration")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// start the movement migrator
	println!("Tasking movement migrator");
	info!("Tasking movement migrator");
	let movement_migrator_for_task = movement_migrator.clone();
	let movement_migrator_task = kestrel::task(async move {
		movement_migrator_for_task.run().await?;
		Ok::<_, anyhow::Error>(())
	});

	// start the movement aptos migrator
	println!("Tasking movement aptos migrator");
	info!("Tasking movement aptos migrator");
	let movement_aptos_migrator_for_task = movement_aptos_migrator.clone();
	let movement_aptos_migrator_task = kestrel::task(async move {
		movement_aptos_migrator_for_task.run().await?;
		Ok::<_, anyhow::Error>(())
	});

	// wait for the rest client to be ready
	println!("Waiting for movement migrator rest client");
	info!("Waiting for movement migrator rest client");
	movement_migrator
		.wait_for_rest_client_ready(tokio::time::Duration::from_secs(10))
		.await
		.context("failed to wait for movement migrator rest client")
		.map_err(|e| {
			println!("Failed to wait for movement migrator rest client: {:?}", e);
			CheckError::Migration(e.into())
		})?;

	// wait for movement aptos migrator to be ready
	println!("Waiting for movement aptos migrator rest client");
	info!("Waiting for movement aptos migrator rest client");
	movement_aptos_migrator
		.wait_for_rest_client_ready(tokio::time::Duration::from_secs(300))
		.await
		.context("failed to wait for movement aptos migrator rest client")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	info!("Running criteria");
	for criterion in criteria {
		criterion
			.satisfies(&movement_migrator, &movement_aptos_migrator)
			.await
			.context("failed to satisfy criterion")
			.map_err(|e| CheckError::Criteria(e.into()))?;
	}

	// kill the movement migrator
	kestrel::end!(movement_migrator_task)
		.context("failed to kill movement migrator")
		.map_err(|e| CheckError::Migration(e.into()))?;
	kestrel::end!(movement_aptos_migrator_task)
		.context("failed to kill movement aptos migrator")
		.map_err(|e| CheckError::Migration(e.into()))?;

	Ok(())
}
