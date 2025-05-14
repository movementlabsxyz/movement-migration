use aptos_config::config::NodeConfig;
use kestrel::State;
use std::path::PathBuf;
pub mod rest_api;
pub mod runtime;
pub use rest_api::RestApi;

use runtime::Runtime;

/// Errors thrown when running [MovementAptos].
#[derive(Debug, thiserror::Error)]
pub enum MovementAptosError {
	#[error("Movement Aptos failed to run with error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Clone)]
pub struct MovementAptos<R>
where
	R: Runtime,
{
	/// The [NodeConfig] for the Aptos node.
	pub node_config: NodeConfig,
	/// The path to the log file.
	pub log_file: Option<PathBuf>,
	/// Whether to create a global rayon pool.
	pub create_global_rayon_pool: std::marker::PhantomData<R>,
	/// The [MovementAptosRestApi] for the Aptos node.
	pub rest_api: State<RestApi>,
}

impl<R> MovementAptos<R>
where
	R: Runtime,
{
	/// If you have something that marks your ability to get a runtime, you can use this.
	pub fn new(node_config: NodeConfig, log_file: Option<PathBuf>, _runtime: R) -> Self {
		Self {
			node_config,
			log_file,
			create_global_rayon_pool: std::marker::PhantomData,
			rest_api: State::new(),
		}
	}

	/// Checks runtime availability and creates a new [MovementAptos].
	pub fn try_new(
		node_config: NodeConfig,
		log_file: Option<PathBuf>,
	) -> Result<Self, anyhow::Error> {
		let runtime = R::try_new()?;
		let movement_aptos = MovementAptos::new(node_config, log_file, runtime);
		Ok(movement_aptos)
	}

	pub fn from_config(config: NodeConfig) -> Result<Self, anyhow::Error> {
		let movement_aptos = MovementAptos::new(config, None, R::try_new()?);
		Ok(movement_aptos)
	}

	/// Borrow the rest api state
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Runs the internal node logic
	pub(crate) fn run_node(&self) -> Result<(), MovementAptosError> {
		aptos_node::start(
			self.node_config.clone(),
			self.log_file.clone(),
			R::create_global_rayon_pool(),
		)
		.map_err(|e| MovementAptosError::Internal(e.into()))?;

		Ok(())
	}

	/// Runs the node and fills state.
	pub async fn run(&self) -> Result<(), MovementAptosError> {
		let rest_api = RestApi {
			rest_api_url: format!(
				"http://{}:{}",
				self.node_config.api.address.ip(),
				self.node_config.api.address.port()
			),
		};

		let runner = self.clone();
		let runner_task = kestrel::task(async move {
			runner.run_node()?;
			Ok::<_, MovementAptosError>(())
		});

		// rest api state
		let rest_api_state = self.rest_api.clone();
		let rest_api_polling = kestrel::task(async move {
			loop {
				// wait for the rest api to be ready
				let response = reqwest::get(rest_api.rest_api_url.clone())
					.await
					.map_err(|e| MovementAptosError::Internal(e.into()))?;
				if response.status().is_success() {
					rest_api_state.write().set(rest_api).await;
					break;
				}
			}

			Ok::<_, MovementAptosError>(())
		});

		// await the runner
		runner_task.await.map_err(|e| MovementAptosError::Internal(e.into()))??;
		rest_api_polling.await.map_err(|e| MovementAptosError::Internal(e.into()))??;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aptos_node::create_single_node_test_config;
	use rand::thread_rng;
	use std::path::Path;

	#[tokio::test(flavor = "multi_thread")]
	async fn test_movement_aptos() -> Result<(), anyhow::Error> {
		// open in a new db
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"movement-aptos-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));

		// create parent dirs
		std::fs::create_dir_all(db_dir.clone())?;

		let rng = thread_rng();
		let node_config = create_single_node_test_config(
			&None,
			&None,
			db_dir.as_path(),
			true,
			false,
			true,
			&aptos_cached_packages::head_release_bundle().clone(),
			rng,
		)?;

		let movement_aptos = MovementAptos::<runtime::TokioTest>::try_new(node_config, None)?;
		let rest_api_state = movement_aptos.rest_api().read().clone();
		movement_aptos.run().await?;
		rest_api_state.wait_for(tokio::time::Duration::from_secs(30)).await?;

		Ok(())
	}
}
