use aptos_config::config::NodeConfig;
use kestrel::State;
use std::path::PathBuf;
pub mod rest_api;
use kestrel::process::{command::Command, ProcessOperations};
pub mod runtime;
use crate::config::{Config, NodeConfigWrapper};
use anyhow::Context;
pub use rest_api::RestApi;
use std::path::Path;

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
	/// The runtime for the Aptos node.
	pub runtime: std::marker::PhantomData<R>,
	/// The [MovementAptosRestApi] for the Aptos node.
	pub rest_api: State<RestApi>,
	/// Whether or not to multiprocess
	pub multiprocess: bool,
	/// The workspace for the multiprocessing should it occur
	///
	/// TODO: this can be tied into a single struct with `multiprocess` and both can be tied in to a type-safe runtime.
	pub workspace: PathBuf,
}

impl<R> MovementAptos<R>
where
	R: Runtime,
{
	/// If you have something that marks your ability to get a runtime, you can use this.
	pub fn new(
		node_config: NodeConfig,
		log_file: Option<PathBuf>,
		_runtime: R,
		multiprocess: bool,
		workspace: PathBuf,
	) -> Self {
		Self {
			node_config,
			log_file,
			runtime: std::marker::PhantomData,
			rest_api: State::new(),
			multiprocess,
			workspace,
		}
	}

	/// Checks runtime availability and creates a new [MovementAptos].
	pub fn try_new(
		node_config: NodeConfig,
		log_file: Option<PathBuf>,
		multiprocess: bool,
	) -> Result<Self, anyhow::Error> {
		let runtime = R::try_new()?;

		// create a .debug dir
		let db_dir = if multiprocess {
			let timestamp = chrono::Utc::now().timestamp_millis();
			let unique_id = uuid::Uuid::new_v4();
			let debug_dir = Path::new(".debug").join(format!(
				"movement-aptos-core-{}-{}",
				timestamp,
				unique_id.to_string().split('-').next().unwrap()
			));
			std::fs::create_dir_all(debug_dir.clone())
				.map_err(|e| MovementAptosError::Internal(e.into()))?;
			debug_dir
		} else {
			Path::new("").to_path_buf()
		};

		let movement_aptos =
			MovementAptos::new(node_config, log_file, runtime, multiprocess, db_dir);
		Ok(movement_aptos)
	}

	pub fn from_config(config: NodeConfig) -> Result<Self, anyhow::Error> {
		let movement_aptos = MovementAptos::try_new(config, None, true)?;
		Ok(movement_aptos)
	}

	/// Borrow the rest api state
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Runs the internal node logic
	pub(crate) async fn run_node_in_thread(&self) -> Result<(), MovementAptosError> {
		// Clone necessary data for the closure
		let node_config = self.node_config.clone();
		let log_file = self.log_file.clone();

		// Spawn the blocking task
		let blocking_task_result = tokio::task::spawn_blocking(move || {
			// This closure runs on a blocking thread
			aptos_node::start(
				node_config,
				log_file,
				R::create_global_rayon_pool(), // Assuming R is in scope and its result is Send
			)
			// The closure should return the direct result from aptos_node::start.
			// The error type from aptos_node::start (let's call it AptosNodeError)
			// needs to be Send + 'static for the closure.
		})
		.await;

		match blocking_task_result {
			Ok(Ok(())) => Ok(()), // aptos_node::start succeeded
			Ok(Err(aptos_node_err)) => {
				// aptos_node::start failed. We need aptos_node_err to be convertible
				// into the Box<dyn Error> for MovementAptosError::Internal.
				Err(MovementAptosError::Internal(aptos_node_err.into()))
			}
			Err(join_err) => {
				// spawn_blocking task failed (e.g., panicked or was cancelled by Tokio)
				Err(MovementAptosError::Internal(Box::new(join_err)))
			}
		}
	}

	pub(crate) async fn run_node_in_process(&self) -> Result<(), MovementAptosError> {
		// write the config to a file
		let config_path = self.workspace.join("config.json");
		let config = Config {
			node_config: NodeConfigWrapper(self.node_config.clone()),
			log_file: self.log_file.clone(),
		};
		std::fs::write(
			config_path.clone(),
			serde_json::to_string(&config).map_err(|e| MovementAptosError::Internal(e.into()))?,
		)
		.map_err(|e| MovementAptosError::Internal(e.into()))?;

		// create node_config data dir
		let data_dir = self.node_config.storage.dir.clone();
		std::fs::create_dir_all(data_dir.clone())
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
			None,
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
			self.run_node_in_thread().await?;
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
				tokio::time::sleep(std::time::Duration::from_secs(1)).await;
				println!("POLLING REST API: {:?}", rest_api);
				// wait for the rest api to be ready
				let response = reqwest::get(rest_api.rest_api_url.clone())
					.await
					.map_err(|e| MovementAptosError::Internal(e.into()))?;

				println!("REST API RESPONSE: {:?}", response);
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

		let movement_aptos = MovementAptos::<runtime::TokioTest>::try_new(node_config, None, true)?;
		let rest_api_state = movement_aptos.rest_api().read().clone();

		let movement_aptos_task = kestrel::task(async move {
			movement_aptos.run().await?;
			Ok::<_, MovementAptosError>(())
		});

		rest_api_state.wait_for(tokio::time::Duration::from_secs(30)).await?;

		println!("ENDING MOVEMENT APTOS");

		kestrel::end!(movement_aptos_task)?;

		Ok(())
	}
}
