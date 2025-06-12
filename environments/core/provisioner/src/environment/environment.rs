use mtma_environment_types::{EnvironmentError, Environmentish};
use mtma_migrator_types::migrator::MovementMigrator;
use std::fmt::Debug;

/// Errors thrown when using the [ProvisionerEnvironment].
#[derive(Debug, thiserror::Error)]
pub enum ProvisionerEnvironmentError {
	#[error("Testing Environment: failed with an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl From<ProvisionerEnvironmentError> for EnvironmentError {
	fn from(e: ProvisionerEnvironmentError) -> Self {
		EnvironmentError::Internal(e.into())
	}
}

pub struct ProvisionerEnvironment {}

impl ProvisionerEnvironment {
	pub fn new() -> Self {
		Self {}
	}
}

impl Environmentish for ProvisionerEnvironment {
	async fn build_movement_migrator(&self) -> Result<MovementMigrator, EnvironmentError> {
		Err(ProvisionerEnvironmentError::Internal(anyhow::anyhow!("not implemented").into()).into())
	}
}
