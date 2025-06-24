use anyhow::Context;
use include_vendor::vendor_workspace;
use kestrel::{
	fulfill::{custom::Custom, Fulfill},
	process::{command::Command, Pipe, ProcessOperations},
	State,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::str::FromStr;
pub mod faucet;
pub mod rest_api;
use movement_core_util::aptos_types::chain_id::ChainId;
use std::path::PathBuf;

use faucet::{Faucet as FaucetApi, ParseFaucet};
use movement_signer_loader::identifiers::SignerIdentifier;
use mtma_types::movement::movement_config::Config as MovementConfig;
use rest_api::{ParseRestApi, RestApi};
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::Arc;
use tracing::info;

vendor_workspace!(MovementWorkspace, "movement");

/// The different overlays that can be applied to the movement runner. s
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Overlay {
	/// The local overlay takes care of setting up the DA sequencer and the full nodd
	Local(Local),
	/// The faucet overlay adds the faucet to whatever orchestration
	Faucet(Faucet),
	/// The fullnode overlay is best used for connecting to an existing network
	Fullnode(Fullnode),
	/// The da squencer overlay prompts the running of the DA seqeuncer
	DaSequencer(DaSequencer),
}

impl Overlay {
	/// Returns the overlay as a string as would be used in a nix command.
	pub fn overlay_arg(&self) -> &str {
		match self {
			Self::Local(local) => local.overlay_arg(),
			Self::Faucet(faucet) => faucet.overlay_arg(),
			Self::Fullnode(fullnode) => fullnode.overlay_arg(),
			Self::DaSequencer(da_sequencer) => da_sequencer.overlay_arg(),
		}
	}
}

/// Errors thrown when parsing an [Overlay] network.
#[derive(Debug, thiserror::Error)]
pub enum OverlayError {
	#[error("invalid overlay selection: {0}")]
	InvalidOverlaySelection(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl FromStr for Overlay {
	type Err = OverlayError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"local" => Self::Local(Local),
			"faucet" => Self::Faucet(Faucet),
			"fullnode" => Self::Fullnode(Fullnode),
			overlay => return Err(OverlayError::InvalidOverlaySelection(overlay.into())),
		})
	}
}

/// The Local network overlay
///
/// Note: we are using a struct for this at the moment as this may benefit from being parameterized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Local;

impl Local {
	/// Returns the overlay as a string as would be used in a nix command.
	///
	/// NOTE: in theory, this could be multiple "."-separated overlays if there are certain overlays which, under parameterization, must be run to support this.
	/// Such was the case at several points in the past.
	pub fn overlay_arg(&self) -> &str {
		"local"
	}
}

/// The Faucet overlay
///
/// Note: we are using a struct for this at the moment as this may benefit from being parameterized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Faucet;

impl Faucet {
	/// Returns the overlay as a string as would be used in a nix command.
	///
	/// NOTE: in theory, this could be multiple "."-separated overlays if there are certain overlays which, under parameterization, must be run to support this.
	/// Such was the case at several points in the past.
	pub fn overlay_arg(&self) -> &str {
		"faucet"
	}
}

/// The Fullnode overlay (generally needed for when you aren't running a local network).
///
/// Note: we are using a struct for this at the moment as this may benefit from being parameterized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fullnode;

impl Fullnode {
	/// Returns the overlay as a string as would be used in a nix command.
	///
	/// NOTE: in theory, this could be multiple "."-separated overlays if there are certain overlays which, under parameterization, must be run to support this.
	/// Such was the case at several points in the past.
	pub fn overlay_arg(&self) -> &str {
		"fullnode"
	}
}

/// The DaSequnecer network overlay
///
/// Note: we are using a struct for this at the moment as this may benefit from being parameterized.
///
/// TODO: we could add some validation of either a DaSequencer URL being available or that selection [Local] requires also selecting [DaSeqeuncer]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DaSequencer;

impl DaSequencer {
	/// Returns the overlay as a string as would be used in a nix command.
	///
	/// NOTE: in theory, this could be multiple "."-separated overlays if there are certain overlays which, under parameterization, must be run to support this.
	/// Such was the case at several points in the past.
	pub fn overlay_arg(&self) -> &str {
		"da-sequencer"
	}
}

/// The set of overlays which shall compose the `movement` application.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Overlays(BTreeSet<Overlay>);

impl Overlays {
	/// Forms an empty [Overlays] set.
	pub fn empty() -> Self {
		Self(BTreeSet::new())
	}

	/// Forms an [Overlays] set from the inner type.
	pub fn new(overlays: BTreeSet<Overlay>) -> Self {
		Self(overlays)
	}

	/// Adds an overlay to the set as a constructor.
	pub fn with(mut self, overlay: Overlay) -> Self {
		self.add(overlay);
		self
	}

	/// Adds an overlay to the set via mutation.
	pub fn add(&mut self, overlay: Overlay) {
		self.0.insert(overlay);
	}

	/// Adds a new set of Overlays (union/join) via mutation
	pub fn add_all(&mut self, overlays: BTreeSet<Overlay>) {
		self.0.extend(overlays);
	}

	/// Gets all of the overlay args
	pub fn to_overlay_args(&self) -> String {
		self.0.iter().map(|o| o.overlay_arg()).collect::<Vec<_>>().join(".")
	}

	/// Converts the overlay set to a Vec<Overlay> which can be useful in type-constrainted contexts such as [clap] args.
	pub fn as_vec(&self) -> Vec<Overlay> {
		self.0.iter().cloned().collect()
	}
}

impl From<BTreeSet<Overlay>> for Overlays {
	fn from(overlays: BTreeSet<Overlay>) -> Self {
		Self(overlays)
	}
}

impl Default for Overlays {
	fn default() -> Self {
		Self::new(BTreeSet::new())
			.with(Overlay::Local(Local))
			// .with(Overlay::Faucet(Faucet))
			.with(Overlay::DaSequencer(DaSequencer))
	}
}

#[derive(Debug, Clone)]
pub enum Workspace {
	Vendor(Arc<MovementWorkspace>),
	Path(PathBuf),
}

impl Workspace {
	pub fn get_workspace_path(&self) -> &Path {
		match self {
			Self::Vendor(workspace) => workspace.get_workspace_path(),
			Self::Path(path) => path,
		}
	}
}

impl From<MovementWorkspace> for Workspace {
	fn from(workspace: MovementWorkspace) -> Self {
		Self::Vendor(Arc::new(workspace))
	}
}

impl From<PathBuf> for Workspace {
	fn from(path: PathBuf) -> Self {
		Self::Path(path)
	}
}

#[derive(Clone)]
pub struct Movement {
	/// The config for the movement runner.
	movement_config: MovementConfig,
	/// The workspace in which [Movement] shall be run.
	workspace: Workspace,
	/// The overlays to apply to the movement runner.
	overlays: Overlays,
	/// Whether to ping the rest api to ensure it is responding to pings.
	ping_rest_api: bool,
	/// The rest api state.
	rest_api: State<RestApi>,
	/// Whether to ping the faucet to ensure it is responding to pings.
	ping_faucet: bool,
	/// The faucet state.
	faucet: State<FaucetApi>,
}

/// Errors thrown when running [Movement].
#[derive(Debug, thiserror::Error)]
pub enum MovementError {
	#[error("movement failed to run with error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Movement {
	/// Creates a new [Movement] with the given workspace and overlays.
	pub fn new(
		movement_config: MovementConfig,
		workspace: Workspace,
		overlays: Overlays,
		ping_rest_api: bool,
		ping_faucet: bool,
	) -> Self {
		Self {
			movement_config,
			workspace,
			overlays,
			ping_rest_api,
			rest_api: State::new(),
			ping_faucet,
			faucet: State::new(),
		}
	}

	/// Tries to form a [Movement] from a well-formed `.movement` directory.
	pub fn try_from_dot_movement_dir(path: PathBuf) -> Result<Self, MovementError> {
		// read the [MovementConfig] from the .movement/config.json file
		let config_path = path.join(".movement/config.json");
		let config =
			std::fs::read_to_string(config_path).map_err(|e| MovementError::Internal(e.into()))?;
		let config: MovementConfig =
			serde_json::from_str(&config).map_err(|e| MovementError::Internal(e.into()))?;

		// set the workspace to the path
		let workspace = Workspace::Path(path);

		Ok(Self::new(config, workspace, BTreeSet::new().into(), true, true))
	}

	/// Creates a new [Movement] with a temporary workspace.
	pub fn try_temp() -> Result<Self, MovementError> {
		let workspace = Workspace::Vendor(Arc::new(
			MovementWorkspace::try_temp().map_err(|e| MovementError::Internal(e.into()))?,
		));
		Ok(Self::new(MovementConfig::default(), workspace, BTreeSet::new().into(), true, true))
	}

	/// Creates a new [Movement] within a debug directory.
	pub fn try_debug() -> Result<Self, MovementError> {
		let workspace = Workspace::Vendor(Arc::new(
			MovementWorkspace::try_debug().map_err(|e| MovementError::Internal(e.into()))?,
		));

		Ok(Self::new(MovementConfig::default(), workspace, BTreeSet::new().into(), true, true))
	}

	/// Creates a new [Movement] within a debug home directory.
	pub fn try_debug_home() -> Result<Self, MovementError> {
		let workspace = Workspace::Vendor(Arc::new(
			MovementWorkspace::try_debug_home().map_err(|e| MovementError::Internal(e.into()))?,
		));

		Ok(Self::new(MovementConfig::default(), workspace, BTreeSet::new().into(), true, true))
	}

	/// Adds an overlay to [Movement].
	pub fn add_overlay(&mut self, overlay: Overlay) {
		self.overlays.add(overlay);
	}

	/// Adds an overlay to [Movement]. (shorthand builder API method for `[Movement::add_overlay]`)
	pub fn with(mut self, overlay: Overlay) -> Self {
		self.add_overlay(overlay);
		self
	}

	/// Sets the overlays for [Movement].
	pub fn set_overlays(&mut self, overlays: Overlays) {
		self.overlays = overlays;
	}

	/// Sets the movement config for [Movement].
	pub fn set_movement_config(&mut self, movement_config: MovementConfig) {
		self.movement_config = movement_config;
	}

	/// Constructs the [MovementConfig] for the container runtime.
	///
	/// NOTE: for the most part, you shouldn't use this method, this is internal to the runner.
	pub fn container_movement_config(&self) -> Result<MovementConfig, MovementError> {
		let mut movement_config: MovementConfig = self.movement_config.clone();

		// todo: we pretty much need this to support the setup that already takes place.
		// set the maptos path
		movement_config.execution_config.maptos_config.chain.maptos_db_path = Some(self.db_dir());
		movement_config.da_db.allow_sync_from_zero = true;
		movement_config.execution_config.maptos_config.da_sequencer.connection_url =
			"http://movement-da-sequencer:30730"
				.parse()
				.context("failed to parse url for da sequencer")
				.map_err(|e| MovementError::Internal(e.into()))?;

		// client
		// rename for the container runtime which uses a `movement-full-node` container
		movement_config
			.execution_config
			.maptos_config
			.client
			.maptos_rest_connection_hostname = "movement-full-node".to_string();
		// rename for the container runtime which uses a `movement-faucet-service` container
		movement_config
			.execution_config
			.maptos_config
			.client
			.maptos_faucet_rest_connection_hostname = "movement-faucet-service".to_string();

		// faucet
		movement_config
			.execution_config
			.maptos_config
			.faucet
			.maptos_rest_connection_hostname = "movement-full-node".to_string();

		// celestia bridge
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.bridge
			.celestia_rpc_connection_hostname = "movement-celestia-appd".to_string();

		// celestia da light node
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.da_light_node
			.celestia_rpc_connection_hostname = "movement-celestia-appd".to_string();

		// celestia da light node bridge
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.da_light_node
			.celestia_websocket_connection_hostname = "movement-celestia-bridge".to_string();

		// appd
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.appd
			.celestia_websocket_connection_hostname = "movement-celestia-bridge".to_string();

		// movement-celestia-da-light-node
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.da_light_node
			.movement_da_light_node_connection_hostname = "movement-celestia-da-light-node".to_string();

		// eth
		movement_config.mcr.eth_connection.eth_rpc_connection_protocol = "http".to_string();
		movement_config.mcr.eth_connection.eth_rpc_connection_hostname = "setup".to_string();
		movement_config.mcr.eth_connection.eth_rpc_connection_port = 8090;
		movement_config.mcr.eth_connection.eth_ws_connection_protocol = "ws".to_string();
		movement_config.mcr.eth_connection.eth_ws_connection_hostname = "setup".to_string();
		movement_config.mcr.eth_connection.eth_ws_connection_port = 8090;
		movement_config.mcr.eth_connection.eth_chain_id = 3073;

		// maybe run local
		movement_config.mcr.maybe_run_local = true;

		// root dir is mounted on container root
		movement_config.syncing.root_dir = "/.movement".to_string().into();

		Ok(movement_config)
	}

	/// Borrows the [RestApi] state.
	pub fn rest_api(&self) -> &State<RestApi> {
		&self.rest_api
	}

	/// Borrows the [FaucetApi] state.
	pub fn faucet(&self) -> &State<FaucetApi> {
		&self.faucet
	}

	/// Runs the movement with the given overlays.
	pub async fn run(&self) -> Result<(), MovementError> {
		// set the CONTAINER_REV environment variable
		std::env::set_var("CONTAINER_REV", movement_core_util::CONTAINER_REV);

		let overlays = self.overlays.to_overlay_args();

		// construct the Rest API fulfiller
		let known_rest_api_listen_url = format!(
			"http://127.0.0.1:{}",
			self.movement_config
				.execution_config
				.maptos_config
				.client
				.maptos_rest_connection_port
				.clone(),
		);
		let rest_api_fulfiller = Custom::new(
			self.rest_api().write(),
			ParseRestApi::new(known_rest_api_listen_url, self.ping_rest_api),
		);

		// construct the Faucet fulfiller
		let known_faucet_listen_url = format!(
			"http://127.0.0.1:{}",
			self.movement_config
				.execution_config
				.maptos_config
				.client
				.maptos_faucet_rest_connection_port
		);
		let faucet_fulfiller = Custom::new(
			self.faucet().write(),
			ParseFaucet::new(known_faucet_listen_url, self.ping_faucet),
		);

		// get the prepared command for the movement task
		let mut command = match &self.workspace {
			// if this is a vendor workspace, we can use the prepared command
			Workspace::Vendor(workspace) => Command::new(
				workspace
					.prepared_command(
						"nix",
						[
							"develop",
							"--command",
							"bash",
							"-c",
							&format!(
							"echo '' > .env && just movement-full-node docker-compose {overlays}"
						),
						],
					)
					.map_err(|e| MovementError::Internal(e.into()))?,
			),
			// otherwise the dir should already be prepared
			Workspace::Path(path) => Command::line(
				"nix",
				[
					"develop",
					"--command",
					"bash",
					"-c",
					&format!("echo $USER && ls -al .movement && echo '' > .env && just movement-full-node docker-compose {overlays}"),
				],
				Some(path),
				false,
				vec![],
				vec![],
			),
		};

		info!(
			"Writing movement config to {:?}",
			self.workspace_path().join(".movement/config.json")
		);
		let container_config = self
			.container_movement_config()
			.map_err(|e| MovementError::Internal(e.into()))?;
		// Write the [MovementConfig] to the workspace path at {workspace_path}/.movement/config.json
		// Use tokio::fs::write to write the config to the file.

		// First create the parent directory if it doesn't exist
		info!("Creating config dir");
		let config_dir = self.workspace_path().join(".movement");
		if !config_dir.exists() {
			std::fs::create_dir_all(&config_dir).map_err(|e| MovementError::Internal(e.into()))?;
		}

		// Then write the config file
		info!("Writing config file");
		let config_path = self.workspace_path().join(".movement/config.json");
		tokio::fs::write(
			&config_path,
			serde_json::to_string(&container_config)
				.map_err(|e| MovementError::Internal(e.into()))?,
		)
		.await
		.map_err(|e| MovementError::Internal(e.into()))?;
		// Set the permissions of the config file to 777
		tokio::fs::set_permissions(&config_path, Permissions::from_mode(0o777))
			.await
			.map_err(|e| MovementError::Internal(e.into()))?;
		info!("Wrote movement config");

		// pipe command output to the rest api fulfiller
		command
			.pipe(
				Pipe::STDOUT,
				rest_api_fulfiller.sender().map_err(|e| MovementError::Internal(e.into()))?,
			)
			.map_err(|e| MovementError::Internal(e.into()))?;

		// pipe command output to the faucet fulfiller
		command
			.pipe(
				Pipe::STDOUT,
				faucet_fulfiller.sender().map_err(|e| MovementError::Internal(e.into()))?,
			)
			.map_err(|e| MovementError::Internal(e.into()))?;

		// start the rest_api_fulfiller
		let rest_api_task =
			rest_api_fulfiller.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// start the faucet fulfiller
		let faucet_task =
			faucet_fulfiller.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// start the command
		let command_task = command.spawn().map_err(|e| MovementError::Internal(e.into()))?;

		// wait for the tasks to finish
		rest_api_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;
		faucet_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;
		command_task
			.await
			.map_err(|e| MovementError::Internal(e.into()))?
			.map_err(|e| MovementError::Internal(e.into()))?;

		Ok(())
	}

	/// Gets the workspace path.
	pub fn workspace_path(&self) -> &Path {
		&self.workspace.get_workspace_path()
	}

	/// Gets the chain id
	pub fn chain_id(&self) -> ChainId {
		self.movement_config.execution_config.maptos_config.chain.maptos_chain_id
	}

	/// Gets the db dir.
	pub fn db_dir(&self) -> PathBuf {
		// this is fixed for now
		self.workspace
			.get_workspace_path()
			.join(".movement")
			.join("maptos")
			.join(self.chain_id().to_string())
			.join(".maptos")
	}

	/// Gets the movement signer identifier
	pub fn movement_signer_identifier(&self) -> &SignerIdentifier {
		&self
			.movement_config
			.execution_config
			.maptos_config
			.chain
			.maptos_private_key_signer_identifier
	}
}

impl Drop for Movement {
	fn drop(&mut self) {
		std::env::set_var("CONTAINER_REV", movement_core_util::CONTAINER_REV);
		// run docker compose down on workspace path
		info!("Dropping movement");
		let workspace_path = self.workspace_path();
		info!("Workspace path: {:?}", workspace_path);
		let overlays = self.overlays.as_vec();
		let mut command = std::process::Command::new("docker");
		command.arg("compose");
		command.arg("-f").arg("docker/compose/movement-full-node/docker-compose.yml");
		for overlay in overlays {
			let overlay_arg = overlay.overlay_arg();
			let compose_file = workspace_path.join(format!(
				"docker/compose/movement-full-node/docker-compose.{}.yml",
				overlay_arg
			));
			command.arg("-f").arg(compose_file);
		}
		command
			.arg("down")
			.env("DOT_MOVEMENT_PATH", workspace_path.join(".movement"))
			.current_dir(workspace_path);
		let result = command.output().map_err(|e| MovementError::Internal(e.into())).unwrap();

		info!("Docker compose down result: {:?}", result);

		if !result.status.success() {
			panic!("Docker compose down failed when dropping movement: {:?}", result);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tokio::time::Duration;

	#[tracing_test::traced_test]
	#[tokio::test]
	async fn test_movement_starts() -> Result<(), anyhow::Error> {
		let mut movement = Movement::try_debug_home()?;
		let rest_api = movement.rest_api().read();
		// let faucet = movement.faucet().read();
		movement.set_overlays(Overlays::default());

		// start movement
		let movement_task = kestrel::task(async move { movement.run().await });

		// wait for the rest api to be ready
		let rest_api = rest_api.wait_for(Duration::from_secs(600)).await?;
		assert_eq!(rest_api.listen_url(), "http://127.0.0.1:30731");

		// wait for the faucet to be ready

		// let faucet = faucet.wait_for(Duration::from_secs(600)).await?;
		// assert_eq!(faucet.listen_url(), "http://0.0.0.0:30732");

		// stop movement
		kestrel::end!(movement_task)?;

		Ok(())
	}
}
