use anyhow::Context;
use bcs_ext::{comparison::BcsEq, conversion::BcsInto};
use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
};

pub struct AccountsEqual;

impl AccountsEqual {
	pub fn new() -> Self {
		Self {}
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self {})
	}
}

impl Criterionish for AccountsEqual {
	async fn satisfies(
		&self,
		movement_migrator: &MovementMigrator,
		movement_aptos_migrator: &MovementAptosMigrator,
	) -> Result<(), CriterionError> {
		let movement_rest_client = movement_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.context(
				"failed to wait for movement migrator rest client while checking accounts equal",
			)
			.map_err(|e| CriterionError::Internal(e.into()))?;
		let movement_aptos_rest_client = movement_aptos_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.context("failed to wait for movement aptos migrator rest client while checking accounts equal")
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let movement_node =
			movement_migrator.node().await.map_err(|e| CriterionError::Internal(e.into()))?;

		println!("Iterating over movement node accounts");
		for account_address_res in movement_node
			.iter_account_addresses(0)
			.map_err(|e| CriterionError::Internal(e.into()))?
		{
			let account_address = match account_address_res
				.context("account address is none")
				.map_err(|e| CriterionError::Internal(e.into()))
			{
				Ok(account_address) => account_address,
				Err(e) => {
					println!("Transaction has no sender: {:?}", e);
					continue;
				}
			};

			println!("Getting movement resource");
			let movement_resource = movement_rest_client
				.get_account_bcs(account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("failed to get account: {:?}", e).into())
				})?
				.into_inner();

			println!("Getting movement aptos address");
			let movement_aptos_account_address =
				account_address.bcs_into().map_err(|e| CriterionError::Internal(e.into()))?;

			println!("Getting movement aptos account resource");
			let aptos_resource = movement_aptos_rest_client
				.get_account_bcs(movement_aptos_account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("Failed to get account: {:?}", e).into())
				})?
				.into_inner();

			println!("Comparing resources");
			movement_resource
				.bcs_eq(&aptos_resource)
				.map_err(|e| CriterionError::Unsatisfied(e.into()))?;

			println!("Finished processing block");
		}

		Ok(())
	}
}
