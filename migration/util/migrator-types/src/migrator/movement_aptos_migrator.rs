use anyhow::Context;
use kestrel::WaitCondition;
use movement_aptos_core::{runtime, MovementAptos};
use mtma_node_types::executor::MovementAptosNode;
use mtma_types::movement_aptos::aptos_config::config::NodeConfig;
use mtma_types::movement_aptos::aptos_rest_client::Client as MovementAptosRestClient;
use mtma_types::movement_aptos::aptos_rest_client::FaucetClient as MovementAptosFaucetClient;

/// An enum supporting different types of runners.
///
/// NOTE: we're starting with just the core runner that has to be embedded.
/// This would expand to include tracking an existing runner. But, in that case, the state files must still be accessible to use derive the executor.
#[derive(Clone)]
pub enum Runner {
	/// [MovementAptos] runner.
	MovementAptos(MovementAptos<runtime::TokioTest>),
}

/// The [MovementAptos] migration struct as would be presented in the criterion.
///
/// This has all of the controls that the a migration implementer should expect to have access to.
#[derive(Clone)]
pub struct MovementAptosMigrator {
	runner: Runner,
}

impl MovementAptosMigrator {
	pub fn new(runner: Runner) -> Self {
		Self { runner }
	}

	/// Runs the migrator.
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		match &self.runner {
			Runner::MovementAptos(movement_aptos) => Ok(movement_aptos.run().await?),
		}
	}

	/// Builds a [MovementAptosMigrator] from a [NodeConfig].
	pub fn try_from_config(config: NodeConfig) -> Result<Self, anyhow::Error> {
		let movement_aptos = MovementAptos::try_from_config(config, None)?;
		let runner = Runner::MovementAptos(movement_aptos);
		Ok(Self::new(runner))
	}

	/// Builds a [MovementAptosMigrator] from a [MovementAptosNode].
	pub fn from_movement_aptos_node(node: MovementAptosNode) -> Result<Self, anyhow::Error> {
		// get the config from the node
		let config = node.node_config()?;
		Ok(Self::try_from_config(config)?)
	}

	/// Rest Api url for the runner.
	pub async fn wait_for_rest_api_url(
		&self,
		condition: impl Into<WaitCondition>,
	) -> Result<String, anyhow::Error> {
		match &self.runner {
			Runner::MovementAptos(movement_aptos) => {
				let rest_api = movement_aptos
					.rest_api()
					.read()
					.wait_for(condition)
					.await
					.context("failed to wait for Movement Aptos rest api")?;
				Ok(rest_api.listen_url().to_string())
			}
		}
	}

	/// Faucet Api url for the runner.
	pub async fn wait_for_faucet_api_url(
		&self,
		condition: impl Into<WaitCondition>,
	) -> Result<String, anyhow::Error> {
		match &self.runner {
			Runner::MovementAptos(movement_aptos) => {
				let faucet_api = movement_aptos
					.faucet_api()
					.read()
					.wait_for(condition)
					.await
					.context("failed to wait for Movement Aptos faucet api")?;
				Ok(faucet_api.listen_url().to_string())
			}
		}
	}

	/// Waits for the rest client to be ready.
	pub async fn wait_for_rest_client_ready(
		&self,
		condition: impl Into<WaitCondition>,
	) -> Result<MovementAptosRestClient, anyhow::Error> {
		let rest_api_url = self.wait_for_rest_api_url(condition).await?;
		let rest_client =
			MovementAptosRestClient::new(rest_api_url.parse().map_err(|e| {
				anyhow::anyhow!("failed to parse Movement Aptos rest api url: {}", e)
			})?);
		Ok(rest_client)
	}
	/// Waits for the rest faucet client to be ready.
	pub async fn wait_for_faucet_client_ready(
		&self,
		condition: impl Into<WaitCondition> + Clone,
	) -> Result<MovementAptosFaucetClient, anyhow::Error> {
		let rest_api_url = self.wait_for_rest_api_url(condition.clone()).await?.parse().map_err(|e| {
			anyhow::anyhow!("failed to parse Movement Aptos rest api url: {}", e)
		})?;
		let faucet_api_url = self.wait_for_faucet_api_url(condition).await?.parse().map_err(|e| {
			anyhow::anyhow!("failed to parse Movement Aptos faucet api url: {}", e)
		})?;
		let faucet_client =
			MovementAptosFaucetClient::new(rest_api_url, faucet_api_url);
		Ok(faucet_client)
	}

	/// Gets a [MovementAptosNode] from the runner.
	pub async fn node(&self) -> Result<MovementAptosNode, anyhow::Error> {
		todo!()
	}

}

impl TryFrom<NodeConfig> for MovementAptosMigrator {
	type Error = anyhow::Error;

	fn try_from(config: NodeConfig) -> Result<Self, Self::Error> {
		Ok(Self::try_from_config(config)?)
	}
}

impl TryFrom<MovementAptosNode> for MovementAptosMigrator {
	type Error = anyhow::Error;

	fn try_from(node: MovementAptosNode) -> Result<Self, Self::Error> {
		Ok(Self::from_movement_aptos_node(node)?)
	}
}
