use migration_e2e_test_types::criterion::{
	CriterionError, Criterionish, MovementAptosE2eClient, MovementE2eClient,
};
use std::collections::HashSet;

pub struct GlobalFeatureCheck;

impl Criterionish for GlobalFeatureCheck {
	async fn satisfies(
		&mut self,
		movement_e2e_client: &MovementE2eClient,
		movement_aptos_e2e_client: &MovementAptosE2eClient,
	) -> Result<(), CriterionError> {
		let mut errors = vec![];
		let expected_active = HashSet::from([73]);
		let expected_inactive = HashSet::<u64>::new();

		for feature_id in 1u64..=100 {
			// Check feature for Aptos executor
			let aptos_active = movement_aptos_e2e_client
				.check_feature_enabled(feature_id)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			// Check feature for Maptos executor
			let maptos_active = movement_e2e_client
				.check_feature_enabled(feature_id)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			if !expected_active.contains(&feature_id) {
				if !maptos_active {
					errors.push(format!(
						"Feature {}: Aptos={}, Maptos={} — expected to be active",
						feature_id, aptos_active, maptos_active
					));
				}
			} else if !expected_inactive.contains(&feature_id) {
				if maptos_active {
					errors.push(format!(
						"Feature {}: Aptos={}, Maptos={} — expected to be inactive",
						feature_id, aptos_active, maptos_active
					));
				}
			} else if aptos_active != maptos_active {
				errors.push(format!(
					"Feature {}: Aptos={}, Maptos={} — expected to match",
					feature_id, aptos_active, maptos_active
				));
			}
		}

		if !errors.is_empty() {
			return Err(CriterionError::Unsatisfied(errors.join("\n").into()));
		}

		Ok(())
	}
}
