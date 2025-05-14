use migration_e2e_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosE2eClient, MovementE2eClient,
};

pub struct GlobalFeatureCheck {
	aptos_client: MovementAptosE2eClient,
}

impl GlobalFeatureCheck {
	pub fn new(aptos_client: MovementAptosE2eClient) -> Self {
		Self { aptos_client }
	}

	pub fn criterion(aptos_client: MovementAptosE2eClient) -> Criterion<Self> {
		Criterion::new(Self { aptos_client })
	}

	pub fn aptos_client(&self) -> &MovementAptosE2eClient {
		&self.aptos_client
	}

	pub fn aptos_client_mut(&mut self) -> &mut MovementAptosE2eClient {
		&mut self.aptos_client
	}
}

impl Criterionish for GlobalFeatureCheck {
	async fn satisfies(
		&mut self,
		movement_e2e_client: &MovementE2eClient,
		movement_aptos_e2e_client: &MovementAptosE2eClient,
	) -> Result<(), CriterionError> {
		let mut mismatches = vec![];
		let unique_features = [73];

		for feature_id in 1u64..=100 {
			// Check feature for Aptos executor
			let aptos_enabled = self
				.aptos_client()
				.check_feature_enabled(feature_id)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			// Check feature for Maptos executor
			let maptos_enabled = movement_aptos_e2e_client
				.check_feature_enabled(feature_id)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			if !unique_features.contains(&feature_id) {
				if !maptos_enabled {
					return Err(CriterionError::Unsatisfied(
						format!("Feature {}: Maptos={} — not enabled", feature_id, maptos_enabled)
							.into(),
					));
				}

				if aptos_enabled != maptos_enabled {
					return Err(CriterionError::Unsatisfied(
						format!(
							"Feature {}: Aptos={}, Maptos={} — mismatch",
							feature_id, aptos_enabled, maptos_enabled
						)
						.into(),
					));
				}
			}

			// Check feature for Movement executor — WARN on differences
			let movement_enabled = movement_e2e_client
				.check_feature_enabled(feature_id)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			if movement_enabled != aptos_enabled {
				mismatches.push(format!(
					"Feature {} differs in Movement (Movement={}, Aptos={})",
					feature_id, movement_enabled, aptos_enabled
				));
			}
		}

		for warning in mismatches {
			eprintln!("Warning: {}", warning);
		}

		Ok(())
	}
}
