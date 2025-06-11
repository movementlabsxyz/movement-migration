use clap::Parser;
use mtma_node_null_core::Config as NullConfig;
use serde::{Deserialize, Serialize};
use slect::Slect;

/// Select migration over the node.
#[derive(Parser, Slect, Deserialize, Serialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct Select {
	/// Extra args to pass to slect API
	#[slect(null = NullConfig)]
	extra_args: Vec<String>,
}

impl select::Select {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let maybe_null = self.select().map_err(|e| anyhow::anyhow!("{}", e))?;

		if let Some(null) = maybe_null {
			let _build = null.build()?;

			// needs an environment for the node to run.
			// How we load this environment is in process of being abstraced.
		}

		Ok(())
	}
}
