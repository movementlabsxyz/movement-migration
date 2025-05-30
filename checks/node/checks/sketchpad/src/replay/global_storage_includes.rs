#[cfg(test)]
pub mod test {

	use mtma_node_preludes::basic::BasicPrelude;
	use mtma_node_replay_core::config::Config as MtmaReplayConfig;
	use mtma_node_test_global_storage_includes_criterion::GlobalStorageIncludes;
	use mtma_node_test_types::{
		check::checked_migration,
		criterion::movement_executor::{MovementNode, MovementOptExecutor},
		prelude::PreludeGenerator,
	};

	#[ignore]
	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_global_storage_includes_null() -> Result<(), anyhow::Error> {
		// form the executor
		let (movement_opt_executor, _temp_dir, private_key, _receiver) =
			MovementOptExecutor::try_generated().await?;
		let mut movement_node = MovementNode::new(movement_opt_executor);

		// form the prelude
		let prelude = BasicPrelude { private_key, chain_id: movement_node.chain_id() }
			.generate()
			.await?;

		// form the migration
		let migration_config = MtmaReplayConfig::default().use_migrated_genesis(true);
		let migration = migration_config.build()?;

		// run the checked migration
		checked_migration(
			&mut movement_node,
			&prelude,
			&migration,
			vec![Box::new(GlobalStorageIncludes::new())],
		)
		.await?;
		Ok(())
	}
}
