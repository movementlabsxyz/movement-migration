use mtma_environment_types::{EnvironmentError, Environmentish};
use mtma_migrator_types::migrator::MovementMigrator;
use std::fmt::Debug;

/// Errors thrown when using the [BoxEnvironment].
#[derive(Debug, thiserror::Error)]
pub enum BoxEnvironmentError {
	#[error("Testing Environment: failed with an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl From<BoxEnvironmentError> for EnvironmentError {
	fn from(e: BoxEnvironmentError) -> Self {
		EnvironmentError::Internal(e.into())
	}
}

pub struct BoxEnvironment {}

impl BoxEnvironment {
	pub fn new() -> Self {
		Self {}
	}
}

impl Environmentish for BoxEnvironment {
	async fn build_movement_migrator(&self) -> Result<MovementMigrator, EnvironmentError> {
		Err(BoxEnvironmentError::Internal(anyhow::anyhow!("not implemented").into()).into())
	}
}
