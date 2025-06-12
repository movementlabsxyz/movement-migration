use mtma_migrator_types::migrator::MovementMigrator;
use std::future::Future;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum EnvironmentError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Environmentish {
	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	fn build_movement_migrator(
		&self,
	) -> impl Future<Output = Result<MovementMigrator, EnvironmentError>>;
}
