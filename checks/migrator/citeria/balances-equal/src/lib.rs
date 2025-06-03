use anyhow::Context;
use bcs_ext::{comparison::BcsEq, conversion::BcsInto};
use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
};
use tracing::info;

pub struct BalancesEqual;

impl BalancesEqual {
	pub fn new() -> Self {
		Self {}
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self {})
	}
}

impl Criterionish for BalancesEqual {
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

		info!("Iterating over movement node accounts");
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
					info!("Transaction has no sender: {:?}", e);
					continue;
				}
			};

			info!("Getting account balance");
			let movement_account_balance = movement_rest_client
				.get_account_balance(account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("failed to get account: {:?}", e).into())
				})?
				.into_inner();

			info!("Getting movement aptos address");
			let movement_aptos_account_address =
				account_address.bcs_into().map_err(|e| CriterionError::Internal(e.into()))?;

			info!("Getting movement aptos account balance");
			let movement_aptos_account_balance = movement_aptos_rest_client
				.view_apt_account_balance(movement_aptos_account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("Failed to get account: {:?}", e).into())
				})?
				.into_inner();

			info!("Comparing balances");
			if u64::from(movement_account_balance.coin.value) != movement_aptos_account_balance {
				return Err(CriterionError::Unsatisfied(
					format!("movement and movement aptos account balances have different values: {:?} != {:?}", movement_account_balance, movement_aptos_account_balance).into(),
				));
			}
			
		}

		Ok(())
	}
}
