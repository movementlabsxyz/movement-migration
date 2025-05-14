use bcs_ext::{comparison::BcsEq, conversion::BcsInto};
use migration_e2e_test_types::criterion::{
	movement_aptos_e2e_client::types::LocalAccount as MovementAptosLocalAccount,
	movement_e2e_client::types::LocalAccount as MovementLocalAccount, Criterion, CriterionError,
	Criterionish, MovementAptosE2eClient, MovementE2eClient,
};

pub struct MaptosTransferLifecycle {
	/// The movement local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	movement_local_account: MovementLocalAccount,
	/// The aptos local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	aptos_local_account: MovementAptosLocalAccount,
}

impl MaptosTransferLifecycle {
	pub fn new(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Self {
		Self { movement_local_account, aptos_local_account }
	}

	pub fn criterion(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Criterion<Self> {
		Criterion::new(Self { movement_local_account, aptos_local_account })
	}

	pub fn movement_local_account(&self) -> &MovementLocalAccount {
		&self.movement_local_account
	}

	pub fn movement_local_account_mut(&mut self) -> &mut MovementLocalAccount {
		&mut self.movement_local_account
	}

	pub fn aptos_local_account(&self) -> &MovementAptosLocalAccount {
		&self.aptos_local_account
	}

	pub fn aptos_local_account_mut(&mut self) -> &mut MovementAptosLocalAccount {
		&mut self.aptos_local_account
	}
}

impl Criterionish for MaptosTransferLifecycle {
	async fn satisfies(
		&mut self,
		movement_e2e_client: &MovementE2eClient,
		movement_aptos_e2e_client: &MovementAptosE2eClient,
	) -> Result<(), CriterionError> {
		for account_address in movement_e2e_client.iter_accounts()? {
			let movement_resource = movement_e2e_client
				.rest_client()
				.get_account_balance(account_address)
				.await
				.map_err(|e| {
					CriterionError::Internal(format!("failed to get account: {:?}", e).into())
				})?
				.into_inner();

			let movement_aptos_account_address =
				account_address.bcs_into().map_err(|e| CriterionError::Internal(e.into()))?;

			let aptos_resource = movement_aptos_e2e_client
				.rest_client()
				.get_account_balance(movement_aptos_account_address, &"0x1::aptos_coin::AptosCoin")
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
