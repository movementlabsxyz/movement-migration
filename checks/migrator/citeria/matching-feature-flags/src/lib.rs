use aptos_rest_client::aptos_api_types;
use movement_client::rest_client::aptos_api_types as maptos_api_types;
use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
};
use serde_json::json;
use std::collections::HashSet;
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

		let mut errors = vec![];
		let expected_active = HashSet::from([73]);
		let expected_inactive = HashSet::<u64>::new();

		let mut aptos_request = aptos_api_types::ViewRequest {
			function: aptos_api_types::EntryFunctionId::from_str("0x1::features::is_enabled")
				.map_err(|e| CriterionError::Internal(e.into()))?,
			type_arguments: vec![aptos_api_types::MoveType::U64],
			arguments: vec![],
		};

		let mut maptos_request = maptos_api_types::ViewRequest {
			function: maptos_api_types::EntryFunctionId::from_str("0x1::features::is_enabled")
				.map_err(|e| CriterionError::Internal(e.into()))?,
			type_arguments: vec![maptos_api_types::MoveType::U64],
			arguments: vec![],
		};

		for feature_id in 1u64..=100 {
			aptos_request.arguments = vec![json!(feature_id)];
			maptos_request.arguments = vec![json!(feature_id)];

			// Check feature for Aptos executor
			let aptos_active = movement_aptos_rest_client
				.view(&aptos_request, None)
				.await
				.map_err(|e| {
					CriterionError::Internal(
						format!("failed to get Aptos feature flag {}: {:?}", feature_id, e).into(),
					)
				})?
				.into_inner();

			let aptos_active = aptos_active.get(0).ok_or_else(|| {
				CriterionError::Internal(
					format!("failed to get Aptos feature flag {}: response is empty", feature_id)
						.into(),
				)
			})?;

			let aptos_active = aptos_active.as_bool().ok_or_else(|| {
				CriterionError::Internal(
					format!(
						"failed to get Aptos feature flag {}: can't convert {:?} into a bool",
						feature_id, aptos_active
					)
					.into(),
				)
			})?;

			// Check feature for Maptos executor
			let maptos_active = movement_rest_client
				.view(&maptos_request, None)
				.await
				.map_err(|e| {
					CriterionError::Internal(
						format!("failed to get Movement feature flag {}: {:?}", feature_id, e)
							.into(),
					)
				})?
				.into_inner();

			let maptos_active = maptos_active.get(0).ok_or_else(|| {
				CriterionError::Internal(
					format!(
						"failed to get Movement feature flag {}: response is empty",
						feature_id
					)
					.into(),
				)
			})?;

			let maptos_active = maptos_active.as_bool().ok_or_else(|| {
				CriterionError::Internal(
					format!(
						"failed to get Movement feature flag {}: can't convert {:?} into a bool",
						feature_id, aptos_active
					)
					.into(),
				)
			})?;

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
