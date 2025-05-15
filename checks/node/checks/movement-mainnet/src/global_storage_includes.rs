#[cfg(test)]
pub mod test {

	use futures::channel::mpsc as futures_mpsc;
	use maptos_execution_util::config::Config as MaptosConfig;
	use mtma_node_test_global_storage_includes_criterion::GlobalStorageIncludes;
	use mtma_node_test_types::{
		check::checked_migration,
		criterion::movement_executor::{MovementNode, MovementOptExecutor},
		prelude::Prelude,
	};
	use movement_syncing::db::DbSync;
	use mtma_node_null_core::config::Config as MtmaNullConfig;
	use sysinfo::Disks;

	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_global_storage_includess_null() -> Result<(), anyhow::Error> {
		let disks = Disks::new_with_refreshed_list();

		let total_disk_space: u64 = disks.iter().map(|disk| disk.total_space()).sum();

		let total_free_space: u64 = disks.iter().map(|disk| disk.available_space()).sum();

		const REQUIRED_SPACE: u64 = 1_000_000_000_000; // 1 TB

		if total_disk_space < REQUIRED_SPACE {
			println!("Device has less than 1 TB of *total* disk space — skipping test (not a viable test platform).");
			return Ok(()); // soft pass
		}

		if total_free_space < REQUIRED_SPACE {
			println!("Device has enough total disk space, but less than 1 TB is free.");
			return Err(anyhow::anyhow!("not enough free disk space"));
		}

		println!("Sufficient disk space available.");

		// sync the db
		let db_sync = DbSync::mainnet_debug();
		db_sync.pull().await?;

		// form the executor
		let (sender, _receiver) = futures_mpsc::channel(100);
		let mut config = MaptosConfig::default();
		config.chain.maptos_db_path = Some(db_sync.destination_db_path().clone());
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
		Ok(())
	}
}
