use anyhow::Context;
use kestrel::WaitCondition;
use movement_core::Movement;
pub use movement_core::{Overlay, Overlays};
use mtma_node_types::executor::MovementNode;
pub use mtma_types::movement::aptos_types::{chain_id::ChainId, state_store::TStateView};
use mtma_types::movement::movement_client::rest_client::Client as MovementRestClient;
pub mod live;
pub use live::LiveMigrator;
use mtma_util::file::copy_dir_recursive;
use std::path::{Path, PathBuf};
use tracing::warn;

/// An enum supporting different types of runners.
///
/// NOTE: we're starting with just the core runner that has to be embedded.
/// This would expand to include tracking an existing runner. But, in that case, the state files must still be accessible to use derive the executor.
#[derive(Clone)]
pub enum Runner {
	/// [Movement] runner.
	Movement(Movement),
	/// [LiveMigrator] runner.
	Live(LiveMigrator),
}

/// The Movement migration struct as would be presented in the criterion.
///
/// This has all the controls that a migration implementer should expect to have access to.
#[derive(Clone)]
pub struct MovementMigrator {
	/// The Movement e2e runner
	runner: Runner,
}

impl MovementMigrator {
	/// Creates a new [MovementMigrator] with the given runner.
	pub fn new(runner: Runner) -> Self {
		Self { runner }
	}

	/// Runs the migrator.
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		match &self.runner {
			Runner::Movement(movement) => Ok(movement.run().await?),
			Runner::Live(live) => Ok(live.run().await?),
		}
	}

	/// Creates a new [MovementMigrator] with temp [Movement] runner.
	pub fn try_temp() -> Result<Self, anyhow::Error> {
		let movement = Movement::try_temp()?;
		Ok(Self::new(Runner::Movement(movement)))
	}

	/// Creates a new [MovementMigrator] with debug [Movement] runner.
	pub fn try_debug() -> Result<Self, anyhow::Error> {
		let movement = Movement::try_debug()?;
		Ok(Self::new(Runner::Movement(movement)))
	}

	/// Create a new [MovementMigrator] with a debug_home [Movement] runner.
	pub fn try_debug_home() -> Result<Self, anyhow::Error> {
		let movement = Movement::try_debug_home()?;
		Ok(Self::new(Runner::Movement(movement)))
	}

	/// Rest Api url for the runner.
	pub async fn wait_for_rest_api_url(
		&self,
		condition: impl Into<WaitCondition>,
	) -> Result<String, anyhow::Error> {
		match &self.runner {
			Runner::Movement(movement) => {
				let rest_api = movement
					.rest_api()
					.read()
					.wait_for(condition)
					.await
					.context("failed to wait for Movement rest api")?;
				Ok(rest_api.listen_url().to_string())
			}
			Runner::Live(live) => live.wait_for_live_rest_api_url().await,
		}
	}

	/// Waits for the rest client to be ready.
	pub async fn wait_for_rest_client_ready(
		&self,
		condition: impl Into<WaitCondition>,
	) -> Result<MovementRestClient, anyhow::Error> {
		let rest_api_url = self.wait_for_rest_api_url(condition).await?;
		let rest_client = MovementRestClient::new(
			rest_api_url
				.parse()
				.map_err(|e| anyhow::anyhow!("failed to parse Movement rest api url: {}", e))?,
		);
		Ok(rest_client)
	}

	/// Produces the [MovementNode] from the runner.
	pub async fn node(&self) -> Result<MovementNode, anyhow::Error> {
		match &self.runner {
			Runner::Movement(movement) => {
				MovementNode::try_from_dir(movement.workspace_path().to_path_buf()).await
			}
			Runner::Live(live) => MovementNode::try_from_dir(live.dir.clone()).await,
		}
	}

	/// Sets the overlays for the runner.
	pub fn set_overlays(&mut self, overlays: Overlays) {
		match &mut self.runner {
			Runner::Movement(movement) => movement.set_overlays(overlays),
			Runner::Live(_live) => {
				warn!("Setting overlays for live migrator is not supported as the runner is already live. If you want to set overlays consider snapshotting the live migrator and using that as a movement runner.");
			}
		}
	}

	/// Gets the dir for the runner
	pub fn dir(&self) -> PathBuf {
		match &self.runner {
			Runner::Movement(movement) => movement.workspace_path().to_path_buf(),
			Runner::Live(live) => live.dir.clone(),
		}
	}

	/// Recursively copies the dir for the runner to the given path
	pub async fn copy_dir(&self, path: &Path) -> Result<(), anyhow::Error> {
		let dir = self.dir();
		copy_dir_recursive(&dir, path).context("failed to copy dir for MovementMigrator")?;
		Ok(())
	}

	/// Snapshots the migrator
	///
	/// NOTE: this can be used to, for example, transition from a live migrator to a movement runner.
	pub async fn snapshot(&self, path: PathBuf) -> Result<Self, anyhow::Error> {
		self.copy_dir(&path).await?;

		Ok(Self::new(Runner::Movement(Movement::try_from_dot_movement_dir(path)?)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_snapshot() -> Result<(), anyhow::Error> {
		let mut migrator = MovementMigrator::try_temp()?;
		migrator.set_overlays(Overlays::default());

		let migrator_for_task = migrator.clone();
		let migrator_task = kestrel::task(async move { migrator_for_task.run().await });

		// wait for the rest api to be ready
		let _rest_api_url =
			migrator.wait_for_rest_api_url(tokio::time::Duration::from_secs(300)).await?;

		// end the migrator task
		kestrel::end!(migrator_task)?;

		// snapshot the migrator
		let snapshot_dir = tempfile::tempdir()?;
		let mut snapshot_migrator = migrator.snapshot(snapshot_dir.path().to_path_buf()).await?;
		snapshot_migrator.set_overlays(Overlays::default());

		// run the snapshot migrator
		let snapshot_migrator_for_task = snapshot_migrator.clone();
		let snapshot_migrator_task =
			kestrel::task(async move { snapshot_migrator_for_task.run().await });

		// wait for the rest api to be ready
		let _rest_api_url = snapshot_migrator
			.wait_for_rest_api_url(tokio::time::Duration::from_secs(300))
			.await?;

		// end the snapshot migrator task
		kestrel::end!(snapshot_migrator_task)?;

		Ok(())
	}
}
