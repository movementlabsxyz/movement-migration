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
			.map_err(|e| CriterionError::Internal(e.into()))?;
		let movement_aptos_rest_client = movement_aptos_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let movement_node =
			movement_migrator.node().await.map_err(|e| CriterionError::Internal(e.into()))?;

		for account_address in movement_node
			.iter_account_addresses(0)
			.map_err(|e| CriterionError::Internal(e.into()))?
		{
			let account_address =
				account_address.map_err(|e| CriterionError::Internal(e.into()))?;

			let movement_resource = movement_rest_client
				.get_account_bcs(account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("failed to get account: {:?}", e).into())
				})?
				.into_inner();

			let movement_aptos_account_address =
				account_address.bcs_into().map_err(|e| CriterionError::Internal(e.into()))?;

			let aptos_resource = movement_aptos_rest_client
				.get_account_bcs(movement_aptos_account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("Failed to get account: {:?}", e).into())
				})?
				.into_inner();

			movement_resource
				.bcs_eq(&aptos_resource)
				.map_err(|e| CriterionError::Unsatisfied(e.into()))?;
		}

		Ok(())
	}
}
