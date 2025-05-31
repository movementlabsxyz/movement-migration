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
	let movement_aptos_migrator = migration
		.migrate(movement_migrator)
		.await
		.context("failed to run migration")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// start the movement migrator
	info!("Tasking movement migrator");
	let movement_migrator_for_task = movement_migrator.clone();
	let movement_migrator_task = kestrel::task(async move {
		movement_migrator_for_task.run().await?;
		Ok::<_, anyhow::Error>(())
	});

	// start the movement aptos migrator
	info!("Tasking movement aptos migrator");
	let movement_aptos_migrator_for_task = movement_aptos_migrator.clone();
	let movement_aptos_migrator_task = kestrel::task(async move {
		movement_aptos_migrator_for_task.run().await?;
		Ok::<_, anyhow::Error>(())
	});

	// wait for the rest client to be ready
	// once we have this, there should also be a config, so we can then kill off the migrator and proceed
	info!("Waiting for movement migrator rest client");
	movement_migrator
		.wait_for_rest_client_ready(tokio::time::Duration::from_secs(120))
		.await
		.context("failed to wait for movement migrator rest client")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// wait for movement aptos migrator to be ready
	info!("Waiting for movement aptos migrator rest client");
	movement_aptos_migrator
		.wait_for_rest_client_ready(tokio::time::Duration::from_secs(120))
		.await
		.context("failed to wait for movement aptos migrator rest client")
		.map_err(|e| CheckError::Migration(e.into()))?;

	// Run the criteria
	info!("Running criteria");
	for criterion in criteria {
		match criterion
			.satisfies(&movement_migrator, &movement_aptos_migrator)
			.await
			.context("failed to satisfy criterion")
			.map_err(|e| CheckError::Criteria(e.into()))
		{
			Ok(()) => {}
			Err(e) => {
				info!("Criterion failed: {:?}", e);
				return Err(e);
			}
		}
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
