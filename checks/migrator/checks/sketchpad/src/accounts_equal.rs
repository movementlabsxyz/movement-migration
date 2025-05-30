#[cfg(test)]
pub mod test {

	use anyhow::Context;
	use mtma_migrator_test_accounts_equal_criterion::AccountsEqual;
	use mtma_migrator_test_types::check::checked_migration;
	use mtma_migrator_types::migrator::{movement_migrator::Overlays, MovementMigrator};
	use mtma_node_null_core::config::Config as MtmaNullConfig;
	use mtma_node_test_types::prelude::Prelude;

	#[tokio::test(flavor = "multi_thread")]
	// #[tracing_test::traced_test]
	async fn test_accounts_equal() -> Result<(), anyhow::Error> {
		// use a scope to ensure everything is dropped
		{
			// Form the migrator.
			let mut movement_migrator = MovementMigrator::try_debug_home()?;
			movement_migrator.set_overlays(Overlays::default());

			// Start the migrator so that it's running in the background.
			// In the future, some migrators may be for already running nodes.
			let movement_migrator_for_task = movement_migrator.clone();
			let movement_migrator_task = kestrel::task(async move {
				movement_migrator_for_task.run().await?;
				Ok::<_, anyhow::Error>(())
			});

			// wait for the rest client to be ready
			// once we have this, there should also be a config, so we can then kill off the migrator and proceed
			movement_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(120))
			.await
			.context(
				"failed to wait for movement migrator rest client while running accounts equal manual prelude",
			)?;

			kestrel::end!(movement_migrator_task)?;

			// Form the prelude.
			// todo: this needs to be updated to use the prelude generator
			let prelude = Prelude::new_empty();

			// Form the migration.
			let migration_config = MtmaNullConfig::default();
			let migration = migration_config.build()?;

			// Run the checked migration.
			let accounts_equal = AccountsEqual::new();
			println!("Running migration");
			match checked_migration(
				&mut movement_migrator,
				&prelude,
				&migration,
				vec![accounts_equal],
			)
			.await
			{
				Ok(()) => {}
				Err(e) => {
					println!("Migration failed: {:?}", e);
					return Err(anyhow::anyhow!("Migration failed: {:?}", e));
				}
			}
			println!("Migration succeeded");
		}

		// exit the test is fine when you only have one test per crate because when cargo test is run across a workspace, it actually multi-processes the tests by crate
		std::process::exit(0);

		Ok(())
	}
}
