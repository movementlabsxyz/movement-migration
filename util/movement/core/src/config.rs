use crate::movement::{Celestia, Eth, Movement, MovementWorkspace, Overlay, Overlays};
use clap::Parser;
use jsonlvar::Jsonl;
use mtma_types::movement::movement_config::Config as MovementConfig;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Errors thrown when parsing an [Eth] network.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("movment-core Config encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Jsonl, Orfile)]
#[clap(help_expected = true)]
pub struct Config {
	/// The movement config.
	// #[clap(flatten)] todo: we would eventually like to flatten the movement config into the main config
	#[clap(long)]
	pub movement_config_string: Option<String>,
	/// Whether to use the setup overlay.
	#[clap(long)]
	pub setup: bool,
	/// Which celestia network to use.
	#[clap(long)]
	pub celestia: Celestia,
	/// Which ethereum network to use.
	#[clap(long)]
	pub eth: Eth,
	/// Whether to use the BiarritizRc1ToPreL1Merge overlay.
	#[clap(long)]
	pub biarritz_rc1_to_pre_l1_merge: bool,
	/// Whether to ping the rest api to ensure it is responding to pings.
	#[clap(long)]
	pub ping_rest_api: bool,
	/// Whether to ping the faucet to ensure it is responding to pings.
	#[clap(long)]
	pub ping_faucet: bool,
}

impl Config {
	/// Serializes a [MovementConfig] struct into a string and places it in the [Config::movement_config_string] field.
	pub fn set_movement_config(&mut self, movement_config: MovementConfig) {
		self.movement_config_string = Some(serde_json::to_string(&movement_config).unwrap());
	}

	/// Builder API for [Config::set_movement_config].
	pub fn set_movement_config_builder(mut self, movement_config: MovementConfig) -> Self {
		self.set_movement_config(movement_config);
		self
	}

	/// Deserializes the movement config from a string.
	pub fn movement_config(&self) -> Result<MovementConfig, ConfigError> {
		match &self.movement_config_string {
			Some(movement_config_string) => Ok(serde_json::from_str(&movement_config_string)
				.map_err(|e| ConfigError::Internal(e.into()))?),
			None => Ok(MovementConfig::default()),
		}
	}

	/// Reififes the [MovementConfig] for the container runtme as is needed in the build
	///
	/// NOTE: for the most part, you shouldn't use this method, this is internal to the runner.
	pub fn build_movement_config(&self) -> Result<MovementConfig, ConfigError> {
		let mut movement_config: MovementConfig = self.movement_config()?;

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

		// movement-celestia-da-light-node
		movement_config
			.celestia_da_light_node
			.celestia_da_light_node_config
			.da_light_node
			.movement_da_light_node_connection_hostname = "movement-celestia-da-light-node".to_string();

		// root dir is mounted on container root
		movement_config.syncing.root_dir = "/.movement".to_string().into();

		Ok(movement_config)
	}

	/// Computes the overlays for the movement runner.
	pub fn overlays(&self) -> Overlays {
		let mut overlays = Overlays::empty();

		overlays.add(Overlay::Celestia(self.celestia));
		overlays.add(Overlay::Eth(self.eth));

		if self.biarritz_rc1_to_pre_l1_merge {
			overlays.add(Overlay::TestMigrateBiarritzRc1ToPreL1Merge);
		}

		overlays
	}

	/// Builds the config into a [Movement] runner.
	pub fn build(&self) -> Result<Movement, ConfigError> {
		Ok(Movement::new(
			self.build_movement_config().map_err(|e| ConfigError::Internal(e.into()))?,
			MovementWorkspace::try_temp().map_err(|e| ConfigError::Internal(e.into()))?,
			self.overlays(),
			self.ping_rest_api,
			self.ping_faucet,
		))
	}
}
