use anyhow::Context;
use mtma_environment_types::{EnvironmentError, Environmentish};
use mtma_migrator_types::migrator::{
	movement_migrator::{LiveMigrator, Runner},
	MovementMigrator,
};
use std::fmt::Debug;
use std::path::PathBuf;

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

pub struct BoxEnvironment {
	/// The rest api url of the box environment.
	pub rest_api_url: String,
	/// The db dir of the box environment.
	pub db_dir: PathBuf,
	/// Whether to isolate the box environment by snapshotting the movement runner and where to store the snapshot.
	pub snapshot_dir: Option<PathBuf>,
}

impl BoxEnvironment {
	pub fn new(rest_api_url: String, db_dir: PathBuf, snapshot_dir: Option<PathBuf>) -> Self {
		Self { rest_api_url, db_dir, snapshot_dir }
	}
}

impl Environmentish for BoxEnvironment {
	async fn build_movement_migrator(&self) -> Result<MovementMigrator, EnvironmentError> {
		// construct the live migrator
		let movement_migrator = MovementMigrator::new(Runner::Live(LiveMigrator::new(
			self.rest_api_url.clone(),
			self.db_dir.clone(),
		)));

		// make sure the live migrator is ready
		let movement_migrator_for_task = movement_migrator.clone();
		let migrator_task = kestrel::task(async move { movement_migrator_for_task.run().await });

		// wait for the rest api to be ready
		let _rest_api_url = movement_migrator
			.wait_for_rest_api_url(tokio::time::Duration::from_secs(300))
			.await
			.context("failed to wait for the rest api to be ready")
			.map_err(|e| BoxEnvironmentError::Internal(e.into()))?;

		// end the migrator task
		kestrel::end!(migrator_task)
			.context("failed to end the migrator task")
			.map_err(|e| BoxEnvironmentError::Internal(e.into()))?;

		// if we're snapshotting the movement_migrator should be a snapshot of the live migrator
		let movement_migrator = match &self.snapshot_dir {
			Some(snapshot_dir) => movement_migrator
				.snapshot(snapshot_dir.clone())
				.await
				.context("failed to snapshot the movement migrator")
				.map_err(|e| BoxEnvironmentError::Internal(e.into()))?,
			None => movement_migrator,
		};

		Ok(movement_migrator)
	}
}
