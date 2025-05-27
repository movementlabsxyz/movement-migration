pub use crate::executor::movement_aptos_executor;
pub use crate::executor::movement_executor;

pub use crate::executor::movement_aptos_executor::MovementAptosNode;
pub use crate::executor::movement_executor::MovementNode;
use std::future::Future;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Migrationish {
	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	fn migrate(
		&self,
		movement_executor: &MovementNode,
	) -> impl Future<Output = Result<MovementAptosNode, MigrationError>>;
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

	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	pub async fn migrate(
		&self,
		movement_executor: &MovementNode,
	) -> Result<MovementAptosNode, MigrationError> {
		self.0.migrate(movement_executor).await
	}
}
