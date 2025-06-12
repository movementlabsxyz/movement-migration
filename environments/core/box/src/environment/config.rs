use crate::BoxEnvironment;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
pub struct Config {}

impl Config {
	/// Builds the [BoxEnvironment] struct from the config.
	///
	/// Note: preserving faillibility here because we may add debug path configuration to the [BoxEnvironment] soon.
	pub fn build(&self) -> Result<BoxEnvironment, EnvironmentConfigError> {
		Ok(BoxEnvironment::new())
	}
}
