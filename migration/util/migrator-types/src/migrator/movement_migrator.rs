use anyhow::Context;
use kestrel::WaitCondition;
use movement_core::Movement;
pub use movement_core::{Overlay, Overlays};
use mtma_node_types::executor::MovementNode;
pub use mtma_types::movement::aptos_types::{chain_id::ChainId, state_store::TStateView};
use mtma_types::movement::movement_client::rest_client::Client as MovementRestClient;

/// An enum supporting different types of runners.
///
/// NOTE: we're starting with just the core runner that has to be embedded.
/// This would expand to include tracking an existing runner. But, in that case, the state files must still be accessible to use derive the executor.
#[derive(Clone)]
pub enum Runner {
	/// [Movement] runner.
	Movement(Movement),
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
		}
	}

	/// Sets the overlays for the runner.
	pub fn set_overlays(&mut self, overlays: Overlays) {
		match &mut self.runner {
			Runner::Movement(movement) => movement.set_overlays(overlays),
		}
	}

	/// Resets the states of the [MovementMigrator] (forces them to be filled again).
	pub async fn reset_states(&self) -> Result<(), anyhow::Error> {
		match &self.runner {
			Runner::Movement(movement) => movement
				.reset_states()
				.await
				.map_err(|e| anyhow::anyhow!("failed to reset movement states: {}", e)),
		}
	}
}
