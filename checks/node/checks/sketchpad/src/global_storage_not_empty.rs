#[cfg(test)]
pub mod test {

	use migration_node_preludes::basic::BasicPrelude;
	use migration_node_test_global_storage_not_empty_criterion::GlobalStorageNotEmpty;
	use migration_node_test_types::{
		check::checked_migration,
		criterion::movement_executor::{MovementNode, MovementOptExecutor},
		prelude::PreludeGenerator,
	};
	use mtma_node_null_core::config::Config as MtmaNullConfig;

	#[tokio::test]
	async fn test_global_storage_injective_null() -> Result<(), anyhow::Error> {
		// form the executor
		let (movement_opt_executor, _temp_dir, private_key, _receiver) =
			MovementOptExecutor::try_generated().await?;
		let mut movement_executor = MovementNode::new(movement_opt_executor);

		// form the prelude
		let prelude_generator =
			BasicPrelude { private_key, chain_id: movement_executor.chain_id() };
		let prelude = prelude_generator.generate().await?;

		// form the migration
		let migration_config = MtmaNullConfig::default();
		let migration = migration_config.build()?;

		// run the checked migration
		checked_migration(
			&mut movement_executor,
			&prelude,
			&migration,
			vec![Box::new(GlobalStorageNotEmpty::new())],
		)
		.await?;

		Ok(())
	}
}
