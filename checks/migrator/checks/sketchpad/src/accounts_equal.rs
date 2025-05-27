#[cfg(test)]
pub mod test {

	use mtma_migrator_test_accounts_equal_criterion::AccountsEqual;
	use mtma_migrator_test_types::check::checked_migration;
	use mtma_migrator_types::migrator::MovementMigrator;
	use mtma_node_null_core::config::Config as MtmaNullConfig;
	use mtma_node_test_types::prelude::Prelude;

	#[ignore] // this is just an example, so it's not expected to pass
	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_global_storage_includes_null() -> Result<(), anyhow::Error> {
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
		let migration_config = MtmaNullConfig::default();
		let migration = migration_config.build()?;

		// Run the checked migration.
		let accounts_equal = AccountsEqual::new();
		checked_migration(&mut movement_migrator, &prelude, &migration, vec![accounts_equal])
			.await?;

		// end the running migrators
		kestrel::end!(movement_migrator_task)?;

		Ok(())
	}
}
