use clap::Parser;
use maptos_execution_util::config::Config as MaptosConfig;
use movement_syncing::db::DbSync;
use mtma_node_null_core::config::Config as MtmaNullConfig;
use mtma_node_null_core::Config;
use mtma_node_test_global_storage_includes_criterion::GlobalStorageIncludes;
use mtma_node_test_types::{
	check::checked_migration,
	criterion::movement_executor::{MovementNode, MovementOptExecutor},
	prelude::Prelude,
};
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub enum State {
	/// Download the state from the s3 bucket
	S3,
	/// Download the state for mainnet
	Mainnet,
	/// Local path to the state
	Local,
}

impl FromStr for State {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"s3" => Self::S3,
			"mainnet" => Self::Mainnet,
			"local" => Self::Local,
			_ => return Err(anyhow::anyhow!("invalid download state")),
		})
	}
}

/// Migrates the node with checks on migration correctness.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct MigrateChecked {
	/// The core config to use.
	#[orfile(config)]
	#[clap(flatten)]
	pub config: Config,
	/// The state to download
	#[clap(long)]
	pub state: State,
	/// The string identifying the download source if necessary
	#[clap(long)]
	pub state_source: Option<String>,
}

impl MigrateChecked {
	async fn impl_execute(&self) -> Result<(), anyhow::Error> {
		// sync the db
		let db_path = match self.state {
			State::Local => {
				// unwrap the state source or error
				let path = PathBuf::from_str(
					self.state_source
						.clone()
						.ok_or(anyhow::anyhow!("state source is required for --state local"))?
						.as_str(),
				)?;
				info!("Using local source: {}", path.display());
				path
			}
			State::S3 => {
				let db_sync = DbSync::mainnet_debug();
				db_sync.pull().await?;
				info!("Downloaded state from S3 to {}", db_sync.destination_db_path().display());
				db_sync.destination_db_path().clone()
			}
			State::Mainnet => {
				let db_sync = DbSync::mainnet_debug();
				db_sync.pull().await?;
				info!(
					"Downloaded state from mainnet to {}",
					db_sync.destination_db_path().display()
				);
				db_sync.destination_db_path().clone()
			}
		};

		// form the executor
		let (sender, _receiver) = tokio::sync::mpsc::unbounded_channel();
		let mut config = MaptosConfig::default();
		config.chain.maptos_db_path = Some(db_path.clone());
		let movement_opt_executor = MovementOptExecutor::try_from_config(config, sender).await?;
		let mut movement_executor = MovementNode::new(movement_opt_executor);

		// empty prelude
		let prelude = Prelude::new_empty();

		// form the migration
		let migration_config = MtmaNullConfig::default();
		let migration = migration_config.build()?;

		// run the checked migration
		checked_migration(
			&mut movement_executor,
			&prelude,
			&migration,
			vec![Box::new(GlobalStorageIncludes::new())],
		)
		.await?;

		info!("Migration completed successfully");
		info!("State path: {}", db_path.display());

		Ok(())
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		// run impl_execute alongside a task that prints "Migration in progress every 5 seconds"
		let me = self.clone();
		let migration_task = tokio::spawn(async move { me.impl_execute().await });
		tokio::spawn(async move {
			loop {
				info!("Migration in progress");
				tokio::time::sleep(std::time::Duration::from_secs(5)).await;
			}
		});

		migration_task.await??;
		Ok(())
	}
}

impl or_file::MigrateChecked {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let inner = self.clone().resolve().await?;
		inner.execute().await
	}
}
