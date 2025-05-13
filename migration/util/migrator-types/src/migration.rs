pub use crate::migrator::movement_aptos_migrator;
pub use crate::migrator::movement_migrator;

pub use crate::migrator::movement_aptos_migrator::{
	MovementAptosMigrator, Runner as MovementAptosRunner,
};
pub use crate::migrator::movement_migrator::MovementMigrator;
use std::future::Future;

use mtma_node_types::migration::Migrationish as ExecutorMigrationish;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Describes a type capable of migration a [MovementMigrator] to a [MovementAptosMigrator].
pub trait Migrationish {
	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	fn migrate(
		&self,
		movement_migrator: &MovementMigrator,
	) -> impl Future<Output = Result<MovementAptosMigrator, MigrationError>>;
}

/// The criterion type simply
pub struct Migration<T>(T)
where
	T: Migrationish;

impl<T> Migration<T>
where
	T: Migrationish,
{
	pub fn new(t: T) -> Self {
		Self(t)
	}

	/// Runs the migration.
	pub async fn migrate(
		&self,
		movement_executor: &MovementMigrator,
	) -> Result<MovementAptosMigrator, MigrationError> {
		self.0.migrate(movement_executor).await
	}
}

impl<T> Migrationish for T
where
	T: ExecutorMigrationish,
{
	async fn migrate(
		&self,
		movement_executor: &MovementMigrator,
	) -> Result<MovementAptosMigrator, MigrationError> {
		let executor = movement_executor
			.executor()
			.await
			.map_err(|e| MigrationError::Internal(e.into()))?;
		let movement_aptos_executor = ExecutorMigrationish::migrate(self, &executor)
			.await
			.map_err(|e| MigrationError::Internal(e.into()))?;
		let movement_aptos = movement_aptos_executor
			.test_movement_aptos_config()
			.map_err(|e| MigrationError::Internal(e.into()))?
			.build()
			.map_err(|e| MigrationError::Internal(e.into()))?;

		let movement_aptos_migrator =
			MovementAptosMigrator::new(MovementAptosRunner::MovementAptos(movement_aptos));

		Ok(movement_aptos_migrator)
	}
}
