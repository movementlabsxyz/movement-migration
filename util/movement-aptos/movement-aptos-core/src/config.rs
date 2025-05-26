pub use aptos_config::config::NodeConfig;
use clap::Parser;
use jsonlvar::Jsonl;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use crate::movement_aptos::{runtime, MovementAptos};
use aptos_node::create_single_node_test_config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfigWrapper(pub(crate) NodeConfig);

impl NodeConfigWrapper {
	pub fn new(node_config: NodeConfig) -> Self {
		Self(node_config)
	}

	pub fn node_config(&self) -> &NodeConfig {
		&self.0
	}

	pub fn into_inner(self) -> NodeConfig {
		self.0
	}
}

impl From<NodeConfigWrapper> for NodeConfig {
	fn from(value: NodeConfigWrapper) -> Self {
		value.into_inner()
	}
}

impl FromStr for NodeConfigWrapper {
	type Err = ConfigError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// if "default" return the default config
		if s == "default" {
			return Ok(NodeConfigWrapper(NodeConfig::default()));
		}

		// otherwise, parse the json
		let node_config: NodeConfig =
			serde_json::from_str(s).map_err(|e| ConfigError::Internal(e.into()))?;
		Ok(NodeConfigWrapper(node_config))
	}
}

/// Errors thrown when parsing an [Eth] network.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
	#[error("movment-core Config encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Jsonl, Orfile)]
#[clap(help_expected = true)]
pub struct Config {
	/// The node config to use.
	#[clap(long)]
	pub node_config: NodeConfigWrapper,

	/// The log file to use.
	#[orfile(config)]
	#[clap(long)]
	pub log_file: Option<PathBuf>,
}

impl Config {
	/// Builds a single validator node test config with the given db dir.
	pub fn test_node_config(db_dir: &PathBuf) -> Result<Self, ConfigError> {
		let rng = rand::thread_rng();

		let node_config = create_single_node_test_config(
			&Some(db_dir.join("config.json")),
			&None,
			db_dir.as_path(),
			true,
			false,
			true,
			&aptos_cached_packages::head_release_bundle().clone(),
			rng,
		)
		.map_err(|e| ConfigError::Internal(e.into()))?;

		Ok(Config { node_config: NodeConfigWrapper(node_config), log_file: None })
	}

	/// Builds the config into a [MovementAptos] runner.
	pub fn build(&self) -> Result<MovementAptos<runtime::TokioTest>, ConfigError> {
		// get the workspace dir
		let workspace_dir =
			self.node_config.node_config().base.working_dir.clone().unwrap_or_default();

		// get the config path
		let config_path = workspace_dir.join("config.yaml");

		// write the config to the config path
		let config_file = File::create(config_path).map_err(|e| ConfigError::Internal(e.into()))?;
		serde_json::to_writer(config_file, &self.node_config.node_config())
			.map_err(|e| ConfigError::Internal(e.into()))?;

		Ok(MovementAptos::<runtime::TokioTest>::new(
			self.node_config.node_config().clone(),
			false,
			workspace_dir,
		))
	}
}
