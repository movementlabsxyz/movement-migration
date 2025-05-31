use crate::movement::{Celestia, Eth, Movement, MovementWorkspace, Overlay, Overlays};
use clap::Parser;
use jsonlvar::Jsonl;
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
}

impl Config {
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
			MovementWorkspace::try_temp().map_err(|e| ConfigError::Internal(e.into()))?,
			self.overlays(),
		))
	}
}
