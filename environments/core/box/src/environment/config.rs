use crate::BoxEnvironment;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum EnvironmentConfigError {
	#[error("failed to build from config: {0}")]
	Build(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The config for the [BoxEnvironment].
///
/// All fields should be easily statically encodable to a CLI argument.
/// This is the frontend for the core API.
#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[clap(help_expected = true)]
pub struct Config {
	/// The rest api url of the box environment.
	#[clap(long)]
	pub rest_api_url: String,
	/// The db dir of the box environment.
	#[clap(long)]
	pub db_dir: PathBuf,
	/// Whether to isolate the box environment by snapshotting the movement runner and where to store the snapshot.
	#[clap(long)]
	pub snapshot_dir: Option<PathBuf>,
}

impl Config {
	/// Builds the [BoxEnvironment] struct from the config.
	///
	/// Note: preserving faillibility here because we may add debug path configuration to the [BoxEnvironment] soon.
	pub fn build(&self) -> Result<BoxEnvironment, EnvironmentConfigError> {
		Ok(BoxEnvironment::new(
			self.rest_api_url.clone(),
			self.db_dir.clone(),
			self.snapshot_dir.clone(),
		))
	}
}
