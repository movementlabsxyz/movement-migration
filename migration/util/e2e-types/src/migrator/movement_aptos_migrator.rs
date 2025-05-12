use anyhow::Context;
use aptos_config::config::NodeConfig;
use aptos_rest_client::Client as MovementAptosRestClient;
use kestrel::WaitCondition;
use movement_aptos_core::MovementAptos;

/// An enum supporting different types of runners.
///
/// NOTE: we're starting with just the core runner that has to be embedded.
/// This would expand to include tracking an existing runner. But, in that case, the state files must still be accessible to use derive the executor.
#[derive(Clone)]
pub enum Runner {
	/// [MovementAptos] runner.
	MovementAptos(MovementAptos),
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

	pub fn from_config(config: NodeConfig) -> Result<Self, anyhow::Error> {
		let movement_aptos = MovementAptos::from_config(config)?;
		let runner = Runner::MovementAptos(movement_aptos);
		Ok(Self::new(runner))
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
					.context("failed to wait for Movement rest api")?;
				Ok(rest_api.listen_url().to_string())
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
}
