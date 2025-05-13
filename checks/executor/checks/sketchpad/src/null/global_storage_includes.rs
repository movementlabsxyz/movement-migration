#[cfg(test)]
pub mod test {

	use migration_executor_preludes::basic::BasicPrelude;
	use migration_executor_test_global_storage_includes_criterion::GlobalStorageIncludes;
	use migration_executor_test_types::{
		check::checked_migration,
		criterion::movement_executor::{MovementExecutor, MovementOptExecutor},
		prelude::PreludeGenerator,
	};
	use mtma_replay_core::config::Config as MtmaNullConfig;

	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_global_storage_includes_null() -> Result<(), anyhow::Error> {
		// form the executor
		let (movement_opt_executor, _temp_dir, private_key, _receiver) =
			MovementOptExecutor::try_generated().await?;
		let mut movement_executor = MovementExecutor::new(movement_opt_executor);

		// form the prelude
		let prelude = BasicPrelude { private_key, chain_id: movement_executor.chain_id() }
			.generate()
			.await?;

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
