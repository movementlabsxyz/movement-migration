#[cfg(test)]
pub mod test {

	use anyhow::Context;
	use mtma_node_test_types::criterion::movement_executor::maptos_opt_executor::aptos_types::account_address::AccountAddress;
	use mtma_migrator_pre_l1_merge_core::config::Config as PreL1MergeConfig;
	use mtma_migrator_test_accounts_equal_criterion::AccountsEqual;
	use mtma_migrator_test_types::check::checked_migration;
	use mtma_migrator_types::migrator::{movement_migrator::{Overlays, MovementMigrator, Runner}};
	use mtma_node_test_types::prelude::Prelude;
	use std::str::FromStr;
	use tracing::info;
	use movement_core::Movement;
	use mtma_types::movement::movement_config::Config as MovementConfig;

	#[tokio::test(flavor = "multi_thread")]
	#[tracing_test::traced_test]
	async fn test_accounts_equal() -> Result<(), anyhow::Error> {
		// use a scope to ensure everything is dropped
		{
			// Create a MovementConfig with the correct port
			let mut movement_config = MovementConfig::default();
			movement_config
				.execution_config
				.maptos_config
				.client
				.maptos_rest_connection_port = 30731;

			// Create a Movement instance with the config
			let movement = Movement::new(
				movement_config,
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
			let mut rest_api_url = movement_migrator
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
			info!("Checking for account existence");
			let mut retries = 0;
			while retries < 10 {
				info!("Attempt {} to get account", retries + 1);
				match rest_client.get_account(AccountAddress::from_str("0xa550c18")?).await {
					Ok(_) => {
						info!("Account found");
						break;
					}
					Err(e) => {
						info!("Account not found, retrying in 1s: {:?}", e);
						tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
						retries += 1;
					}
				}
			}
			if retries >= 10 {
				info!("Failed to find account after 10 retries");
			}

			// Form the prelude.
			info!("Creating empty prelude");
			let prelude = Prelude::new_empty();

			// Form the migration.
			let migration_config = PreL1MergeConfig::default();
			let migration = migration_config.build()?;

			// Run the checked migration.
			let accounts_equal = AccountsEqual::new();
			info!("Running migration");
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
					info!("Migration failed: {:?}", e);
					return Err(anyhow::anyhow!("Migration failed: {:?}", e));
				}
			}
			info!("Migration succeeded");

			// Let the task run in the background and let the Drop implementation handle the shutdown
			drop(movement_migrator_task);
		}

		// exit the test is fine when you only have one test per crate because when cargo test is run across a workspace, it actually multi-processes the tests by crate
		std::process::exit(0);

		Ok(())
	}
}
