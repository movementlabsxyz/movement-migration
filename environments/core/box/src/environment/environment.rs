use anyhow::Context;
use movement_core::Overlays;
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
		// Form the migrator.
		let mut movement_migrator =
			MovementMigrator::try_temp().map_err(|e| BoxEnvironmentError::Internal(e.into()))?;
		movement_migrator.set_overlays(Overlays::default());

		// Start the migrator so that it's running in the background.
		// In the future, some migrators may be for already running nodes.
		let movement_migrator_for_task = movement_migrator.clone();
		let movement_migrator_task = kestrel::task(async move {
			movement_migrator_for_task.run().await?;
			Ok::<_, anyhow::Error>(())
		});

		// wait for the rest client to be ready
		// once we have this, there should also be a config, so we can then kill off the migrator and proceed
		movement_migrator
		.wait_for_rest_client_ready(tokio::time::Duration::from_secs(600)) // we wait for up to ten minutes because the nix flake in .vendors/movementcan be a bit slow the first time
		.await
		.context(
			"failed to wait for movement migrator rest client while running accounts equal manual prelude",
		).map_err(|e| BoxEnvironmentError::Internal(e.into()))?;

		kestrel::end!(movement_migrator_task)
			.context("failed to end movement migrator task")
			.map_err(|e| BoxEnvironmentError::Internal(e.into()))?;

		Ok(movement_migrator)
	}
}
