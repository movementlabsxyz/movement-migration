use kestrel::State;
use mtma_types::movement_aptos::aptos_config::config::NodeConfig;
use std::path::PathBuf;
pub mod rest_api;
use kestrel::process::{command::Command, ProcessOperations};
pub mod faucet_api;
pub mod runtime;
use anyhow::Context;
pub use rest_api::RestApi;
pub use faucet_api::FaucetApi;
use runtime::Runtime;
use std::marker::PhantomData;
use tracing::{info, debug, warn};

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
	/// The [NodeConfig]
	pub node_config: NodeConfig,
	/// The faucet port
	pub faucet_port: u16,
	/// Whether or not to multiprocess
	pub multiprocess: bool,
	/// The workspace for the multiprocessing should it occur
	///
	/// The [NodeConfig] will be stored in the workspace at `config.json`.
	pub workspace: PathBuf,
	/// The rest api state
	pub rest_api: State<RestApi>,
	/// The faucet api state
	pub faucet_api: State<FaucetApi>,
	/// The marker for the runtime
	pub runtime: PhantomData<R>,
}

impl<R> MovementAptos<R>
where
	R: Runtime,
{
	/// If you have something that marks your ability to get a runtime, you can use this.
	pub fn new(node_config: NodeConfig, faucet_port: u16, multiprocess: bool, workspace: PathBuf) -> Self {
		Self { node_config, faucet_port, multiprocess, workspace, rest_api: State::new(), faucet_api: State::new(), runtime: PhantomData }
	}

	/// Constructs a new [MovementAptos] from a [NodeConfig].
	pub fn try_from_config(config_path: NodeConfig, faucet_port: Option<u16>) -> Result<Self, MovementAptosError> {

		let faucet_port = match faucet_port {
			Some(port) => port,
			None => portpicker::pick_unused_port().context("Failed to pick unused port").map_err(|e| MovementAptosError::Internal(e.into()))?
		};

		let workspace = config_path
			.base
			.working_dir
			.clone()
			.context("Working directory not set")
			.map_err(|e| MovementAptosError::Internal(e.into()))?;
		Ok(Self::new(config_path, faucet_port, true, workspace))
	}

	/// Borrow sthe rest api state
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Borrow the faucet api state
	pub fn faucet_api(&self) -> &State<FaucetApi> {
		&self.faucet_api
	}

	/// Borrows the [NodeConfig]
	pub fn node_config(&self) -> &NodeConfig {
		&self.node_config
	}

	/// Runs the internal node logic
	pub(crate) async fn run_node_in_thread(&self) -> Result<(), MovementAptosError> {
		info!("Running node in thread");
		// Clone necessary data for the closure
		let node_config = self.node_config.clone();
		let test_dir = self.workspace.clone();

		// Spawn the blocking task
		tokio::task::spawn_blocking(move || {
			// This closure runs on a blocking thread
			aptos_node::start_test_environment_node(
				node_config,
				test_dir,
				R::create_global_rayon_pool(), // Assuming R is in scope and its result is Send
			)
			// The closure should return the direct result from aptos_node::start.
			// The error type from aptos_node::start (let's call it AptosNodeError)
			// needs to be Send + 'static for the closure.
		})
		.await
		.map_err(|e| MovementAptosError::Internal(e.into()))?
		.map_err(|e| MovementAptosError::Internal(e.into()))?;

		Ok(())
	}

	/// Runs the node in a new process, effectively passing the config to a process which runs [run_node_in_thread] from above.
	pub(crate) async fn run_node_in_process(&self) -> Result<(), MovementAptosError> {
		info!("Running node in process");

		use std::os::unix::fs::MetadataExt;
		let meta = std::fs::metadata(&self.workspace)
			.map_err(|e| MovementAptosError::Internal(e.into()))?;
		info!("UID: {}, GID: {}, Mode: {:o}", meta.uid(), meta.gid(), meta.mode());

		// write the node config to the workspace at `config.yaml`
		let serialized_node_config = serde_yaml::to_string(&self.node_config)
			.map_err(|e| MovementAptosError::Internal(e.into()))?;
		let config_path = self.workspace.join("config.yaml");
		tokio::fs::write(config_path.clone(), serialized_node_config)
			.await
			.map_err(|e| MovementAptosError::Internal(e.into()))?;

		// note: we could grab the entirety of the run-localnet [Args] struct and expose it, replacing the test dir and config path as is reasonable. 
		// But, we will do that ad hoc.
		let command = Command::line(
			"aptos",
			vec![
				"node",
				"run-localnet",
				"--test-dir",
				&self.workspace.to_string_lossy(),
				"--config-path",
				&config_path.to_string_lossy(),
				"--faucet-port",
				&self.faucet_port.to_string(),
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
			self.run_node_in_thread().await?;
		}

		Ok(())
	}

	/// Runs the node and fills state.
	pub async fn run(&self) -> Result<(), MovementAptosError> {
		let rest_api = RestApi { rest_api_url: format!("http://{}", self.node_config.api.address) };
		let faucet_api = FaucetApi { faucet_api_url: format!("http://127.0.0.1:{}", self.faucet_port) };

		let runner = self.clone();
		let runner_task = kestrel::task(async move {
			runner.run_node().await?;
			Ok::<_, MovementAptosError>(())
		});

		// rest api state
		let rest_api_state = self.rest_api.clone();
		let rest_api_polling = kestrel::task(async move {
			// sometimes if you poll too soon, it will prevent the node from starting
			tokio::time::sleep(std::time::Duration::from_secs(5)).await;
			loop {
				tokio::time::sleep(std::time::Duration::from_secs(1)).await;
				debug!("Polling rest api: {:?}", rest_api);
				// wait for the rest api to be ready
				match reqwest::get(rest_api.rest_api_url.clone())
					.await
					.map_err(|e| MovementAptosError::Internal(e.into()))
				{
					Ok(response) => {
						debug!("Received response from rest api: {:?}", response);
						if response.status().is_success() {
							rest_api_state.write().set(rest_api).await;
							break;
						} else {
							warn!("Failed to poll rest api: {:?}", response);
						}
					}
					Err(e) => {
						warn!("Encountered error while polling rest api: {:?}", e);
					}
				}
			}

			Ok::<_, MovementAptosError>(())
		});

		// faucet api state
		let faucet_api_state = self.faucet_api.clone();
		let faucet_api_polling = kestrel::task(async move {
			loop {
				tokio::time::sleep(std::time::Duration::from_secs(1)).await;
				debug!("Polling faucet api: {:?}", faucet_api);
				// wait for the faucet api to be ready
				match reqwest::get(faucet_api.faucet_api_url.clone())
					.await
					.map_err(|e| MovementAptosError::Internal(e.into()))
				{
					Ok(response) => {
						debug!("Received response from faucet api: {:?}", response);
						if response.status().is_success() {
							faucet_api_state.write().set(faucet_api).await;
							break;
						} else {
							warn!("Failed to poll faucet api: {:?}", response);
						}
					}
					Err(e) => {
						warn!("Encountered error while polling faucet api: {:?}", e);
					}
				}
			}

			Ok::<_, MovementAptosError>(())
		});

		// await the runner
		runner_task.await.map_err(|e| MovementAptosError::Internal(e.into()))??;
		rest_api_polling.await.map_err(|e| MovementAptosError::Internal(e.into()))??;
		faucet_api_polling.await.map_err(|e| MovementAptosError::Internal(e.into()))??;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mtma_types::movement_aptos::aptos_node::create_single_node_test_config;
	use rand::thread_rng;
	use std::path::Path;

	#[tracing_test::traced_test]
	#[tokio::test]
	async fn test_movement_aptos() -> Result<(), anyhow::Error> {
		// open in a new db
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let working_dir = std::env::current_dir()?.join(Path::new(".debug")).join(format!(
			"movement-aptos-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
		let db_dir = working_dir.join("0");
		let faucet_port = portpicker::pick_unused_port().context("Failed to pick unused port").map_err(|e| MovementAptosError::Internal(e.into()))?;

		// create parent dirs
		{
			tokio::fs::create_dir_all(db_dir.clone()).await?;
		}

		let rng = thread_rng();
		let mut node_config = create_single_node_test_config(
			&None,
			&None,
			working_dir.as_path(),
			true,
			false,
			false,
			&aptos_cached_packages::head_release_bundle().clone(),
			rng,
		)?;
		node_config.base.working_dir = Some(working_dir.clone());
		node_config.storage.dir = db_dir.clone();

		let movement_aptos =
			MovementAptos::<runtime::Delegated>::new(node_config, faucet_port, true, working_dir);

		// extract the states for which we will wait
		let rest_api_state = movement_aptos.rest_api().read().clone();
		let faucet_api_state = movement_aptos.faucet_api().read().clone();

		let movement_aptos_task = kestrel::task(async move {
			movement_aptos.run().await?;
			Ok::<_, MovementAptosError>(())
		});

		rest_api_state.wait_for(tokio::time::Duration::from_secs(120)).await?;
		faucet_api_state.wait_for(tokio::time::Duration::from_secs(120)).await?;

		kestrel::end!(movement_aptos_task)?;

		Ok(())
	}
}
