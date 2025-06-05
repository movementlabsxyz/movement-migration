use aptos_rest_client::aptos_api_types;
use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
};
use std::str::FromStr;

pub struct GlobalFeatureCheck;

impl GlobalFeatureCheck {
	pub fn new() -> Self {
		Self {}
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self {})
	}
}

impl Criterionish for GlobalFeatureCheck {
	async fn satisfies(
		&self,
		_movement_migrator: &MovementMigrator,
		movement_aptos_migrator: &MovementAptosMigrator,
	) -> Result<(), CriterionError> {
		let movement_aptos_rest_client = movement_aptos_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let request = aptos_api_types::ViewRequest {
			function: aptos_api_types::EntryFunctionId::from_str(
				"0x1::aptos_governance::get_required_proposer_stake",
			)
			.map_err(|e| CriterionError::Internal(e.into()))?,
			type_arguments: vec![],
			arguments: vec![],
		};

		let required_proposer_stake = movement_aptos_rest_client
			.view(&request, None)
			.await
			.map_err(|e| {
				CriterionError::Internal(
					format!(
						"failed to get Aptos required proposer state (request failed): {:?}",
						e
					)
					.into(),
				)
			})?
			.into_inner();

		let required_proposer_stake = required_proposer_stake.get(0).ok_or_else(|| {
			CriterionError::Internal(
				"failed to get Aptos required proposer state: response is empty"
					.to_string()
					.into(),
			)
		})?;

		let required_proposer_stake = required_proposer_stake.as_u64().ok_or_else(|| {
			CriterionError::Internal(
				format!(
					"failed to get Aptos required proposer state: can't convert {:?} into an u64",
					required_proposer_stake
				)
				.into(),
			)
		})?;

		assert!(required_proposer_stake > 0);

		Ok(())
	}
}
