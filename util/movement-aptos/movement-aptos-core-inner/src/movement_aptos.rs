use aptos_config::config::NodeConfig;
use kestrel::State;
use std::path::PathBuf;
pub mod rest_api;
use anyhow::Context;
use kestrel::process::{command::Command, ProcessOperations};
pub use rest_api::RestApi;
use std::path::Path;

/// Errors thrown when running [MovementAptos].
#[derive(Debug, thiserror::Error)]
pub enum MovementAptosError {
	#[error("Movement Aptos failed to run with error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Clone)]
pub struct MovementAptos {
	/// The [NodeConfig] for the Aptos node.
	pub node_config: NodeConfig,
	/// The path to the log file.
	pub log_file: Option<PathBuf>,
	/// Whether to create a global rayon pool.
	pub create_global_rayon_pool: bool,
	/// The [MovementAptosRestApi] for the Aptos node.
	pub rest_api: State<RestApi>,
	/// Whether or not to multiprocess
	pub multiprocess: bool,
	/// The workspace for the multiprocessing should it occur
	///
	/// TODO: this can be tied into a single struct with `multiprocess` and both can be tied in to a type-safe runtime.
	pub workspace: PathBuf,
}

impl MovementAptos {
	pub fn try_new(
		node_config: NodeConfig,
		log_file: Option<PathBuf>,
		create_global_rayon_pool: bool,
		multiprocess: bool,
	) -> Result<Self, MovementAptosError> {
		// create a .debug dir
		let timestamp = chrono::Utc::now().timestamp_millis();
		let unique_id = uuid::Uuid::new_v4();
		let debug_dir = Path::new(".debug").join(format!(
			"movement-aptos-core-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
		std::fs::create_dir_all(debug_dir.clone())
			.map_err(|e| MovementAptosError::Internal(e.into()))?;
		Ok(Self {
			node_config,
			log_file,
			create_global_rayon_pool,
			rest_api: State::new(),
			multiprocess,
			workspace: debug_dir,
		})
	}

	pub fn from_config(config: NodeConfig) -> Result<Self, anyhow::Error> {
		let movement_aptos = MovementAptos::try_new(config, None, false, false)?;
		Ok(movement_aptos)
	}

	/// Borrow the rest api state
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Runs the internal node logic
	pub(crate) fn run_node_in_thread(&self) -> Result<(), MovementAptosError> {
		aptos_node::start(
			self.node_config.clone(),
			self.log_file.clone(),
			self.create_global_rayon_pool,
		)
		.map_err(|e| MovementAptosError::Internal(e.into()))?;

		Ok(())
	}

	pub(crate) async fn run_node_in_process(&self) -> Result<(), MovementAptosError> {
		// write the config to a file
		let config_path = self.workspace.join("config.json");
		std::fs::write(
			config_path.clone(),
			serde_json::to_string(&self.node_config)
				.map_err(|e| MovementAptosError::Internal(e.into()))?,
		)
		.map_err(|e| MovementAptosError::Internal(e.into()))?;

		// spawn the node in a new process
		let command = Command::line(
			"movement-aptos",
			vec![
				"run",
				"using",
				"--config-path",
				config_path
					.to_str()
					.context("Failed to convert config path to str")
					.map_err(|e| MovementAptosError::Internal(e.into()))?,
			],
			Some(&self.workspace),
			false,
			vec![],
			vec![],
		);
		command.run().await.map_err(|e| MovementAptosError::Internal(e.into()))?;

		Ok(())
	}

	pub(crate) async fn run_node(&self) -> Result<(), MovementAptosError> {
		if self.multiprocess {
			self.run_node_in_process().await?;
		} else {
			self.run_node_in_thread()?;
		}

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
			runner.run_node().await?;
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

	#[tokio::test]
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

		let movement_aptos = MovementAptos::try_new(node_config, None, false, false)?;
		let rest_api_state = movement_aptos.rest_api().read().clone();
		movement_aptos.run().await?;
		rest_api_state.wait_for(tokio::time::Duration::from_secs(30)).await?;

		Ok(())
	}
}
