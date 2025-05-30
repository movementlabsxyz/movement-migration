#[cfg(test)]
pub mod test {

	use mtma_migrator_pre_l1_merge_core::config::Config as PreL1MergeConfig;
	use mtma_migrator_test_types::check::checked_migration;
	use mtma_migrator_types::migrator::MovementMigrator;
	use mtma_node_null_core::config::Config as MtmaNullConfig;
	use mtma_node_test_types::prelude::Prelude;

	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_pre_l1_merge() -> Result<(), anyhow::Error> {
		// Form the migrator.
		let mut movement_migrator = MovementMigrator::try_temp()?;

		// Start the migrator so that it's running in the background.
		// In the future, some migrators may be for already running nodes.
		let movement_migrator_for_task = movement_migrator.clone();
		let movement_migrator_task = kestrel::task(async move {
			movement_migrator_for_task.run().await?;
			Ok::<_, anyhow::Error>(())
		});

		// Form the prelude.
		// todo: this needs to be updated to use the prelude generator
		let prelude = Prelude::new_empty();

		// Form the migration.
		let migration_config = PreL1MergeConfig::default();
		let migration = migration_config.build()?;

		// Run the checked migration.
		checked_migration(&mut movement_migrator, &prelude, &migration, vec![]).await?;

		// WITHIN THE CRITERION, you'd be able to do similar background startup and processing to the above. The [Migrationish] trait will give you a [MovementAptosMigrator] which can spawn the runner in the background.
		//
		// Note: WITHIN THE CRITERION, you'll be able to do something like the below.
		// let movement_aptos_migrator_for_task = movement_aptos_migrator.clone();
		// let movement_aptos_migrator_task = kestrel::task(async move {
		// 	movement_aptos_migrator_for_task.run().await?;
		// 	Ok::<_, anyhow::Error>(())
		// });
		// // TODO: SOMETHING
		// kestrel::end!(movement_aptos_migrator_task)?;

		// end the running migrators
		kestrel::end!(movement_migrator_task)?;

		Ok(())
	}
}
