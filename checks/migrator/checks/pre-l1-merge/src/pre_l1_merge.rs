#[cfg(test)]
pub mod test {

	use anyhow::Context;
	use movement_core::Movement;
	use mtma_migrator_pre_l1_merge_core::config::Config as PreL1MergeConfig;
	use mtma_migrator_test_accounts_equal_criterion::AccountsEqual;
	use mtma_migrator_test_types::check::checked_migration;
	use mtma_migrator_types::migrator::movement_migrator::{MovementMigrator, Overlays, Runner};
	use mtma_node_test_types::prelude::Prelude;
	use mtma_types::movement::aptos_sdk::types::account_config::aptos_test_root_address;
	use mtma_types::movement::movement_config::Config as MovementConfig;
	use tracing::info;

	#[tokio::test(flavor = "multi_thread")]
	#[tracing_test::traced_test]
	async fn test_pre_l1_merge() -> Result<(), anyhow::Error> {
		// use a scope to ensure everything is dropped
		{
			// Create a MovementConfig with the correct port
			let mut movement_config = MovementConfig::default();
			movement_config
				.execution_config
				.maptos_config
				.client
				.maptos_rest_connection_port = 30731;

			// Get the account address from the same MovementConfig
			let account_address = aptos_test_root_address();

			// Create a Movement instance with the config
			let movement = Movement::new(
				movement_config.clone(),
				movement_core::MovementWorkspace::try_temp()?,
				Overlays::default(),
				true,
				true,
			);

			// Form the migrator with the configured Movement instance
			let mut movement_migrator = MovementMigrator::new(Runner::Movement(movement));

			// Start the migrator so that it's running in the background.
			// In the future, some migrators may be for already running nodes.
			let movement_migrator_for_task = movement_migrator.clone();
			let movement_migrator_task = kestrel::task(async move {
				movement_migrator_for_task.run().await?;
				Ok::<_, anyhow::Error>(())
			});

			// Wait for the Celestia light node to be ready
			tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

			// wait for the rest client to be ready
			info!("Waiting for REST client to be ready (timeout: 600s)");
			let rest_api_url = movement_migrator
				.wait_for_rest_api_url(tokio::time::Duration::from_secs(600))
				.await?;
			info!("REST API URL: {}", rest_api_url);

			movement_migrator
				.wait_for_rest_client_ready(tokio::time::Duration::from_secs(600))
				.await
				.context(
					"failed to wait for movement migrator rest client while running accounts equal manual prelude",
				)?;
			info!("REST client is ready");

			// Wait for the account to be created
			info!("Getting REST client for account check (timeout: 30s)");
			let rest_client = movement_migrator
				.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
				.await
				.context("failed to get rest client")?;
			info!("Checking for account existence, address: {}", account_address);
			match rest_client.get_account(account_address).await {
				Ok(_) => {
					info!("Account found");
				}
				Err(e) => {
					info!("Account not found: {:?}", e);
				}
			}

			// Form the prelude.
			info!("Creating empty prelude");
			let prelude = Prelude::new_empty();

			// Form the migration.
			let mut migration_config = PreL1MergeConfig::default();
			migration_config.account_address = Some(account_address);
			let migration = migration_config.build()?;

			// Run the checked migration (skipping the accounts equal check).
			info!("Running migration");
			match checked_migration(
				&mut movement_migrator,
				&prelude,
				&migration,
				Vec::<AccountsEqual>::new(),
			)
			.await
			{
				Ok(()) => {}
				Err(e) => {
					info!("Migration failed: {:?}", e);
					return Err(anyhow::anyhow!("Migration failed: {:?}", e));
				}
			}
			info!("Migration succeeded");

			kestrel::end!(movement_migrator_task)?;
		}

		// exit the test is fine when you only have one test per crate because when cargo test is run across a workspace, it actually multi-processes the tests by crate
		std::process::exit(0);

		Ok(())
	}
}
