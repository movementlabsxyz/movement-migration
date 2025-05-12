use aptos_config::config::NodeConfig;
use clap::Parser;
use jsonlvar::Jsonl;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

use crate::movement_aptos::MovementAptos;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfigWrapper(NodeConfig);

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

	/// Whether to create a global rayon pool.
	#[orfile(config)]
	#[clap(long)]
	pub create_global_rayon_pool: bool,
}

impl Config {
	/// Builds the config into a [MovementAptos] runner.
	pub fn build(&self) -> Result<MovementAptos, ConfigError> {
		Ok(MovementAptos::new(
			self.node_config.0.clone(),
			self.log_file.clone(),
			self.create_global_rayon_pool,
		))
	}
}
