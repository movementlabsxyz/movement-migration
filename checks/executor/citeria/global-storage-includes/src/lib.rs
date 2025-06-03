use migration_executor_test_types::criterion::movement_aptos_executor::TStateView as _;
use migration_executor_test_types::criterion::movement_executor::TStateView as _;
use migration_executor_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosExecutor, MovementExecutor,
};
use migration_executor_test_types::criterion::movement_aptos_executor::aptos_types::state_store::state_key::StateKey as MovementAptosStateKey;
use tracing::debug;
pub struct GlobalStorageIncludes;

impl GlobalStorageIncludes {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl GlobalStorageIncludes {
	fn satisfies_for_version(
		&self,
		movement_executor: &MovementExecutor,
		maptos_executor: &MovementAptosExecutor,
		movement_ledger_version: u64,
	) -> Result<(), CriterionError> {
		// get the latest state view from the movement executor
		let movement_state_view = movement_executor
			.state_view_at_version(Some(movement_ledger_version))
			.map_err(|e| CriterionError::Internal(e.into()))?;

		// get the latest state view from the maptos executor
		let maptos_state_view = maptos_executor
			.state_view_at_version(Some(movement_ledger_version))
			.map_err(|e| CriterionError::Internal(e.into()))?;

		// the movement state view is the domain, so the maptos state view is the codomain
		let movement_global_state_keys_iterator =
			movement_executor.global_state_keys_from_version(None);
		let movement_global_state_keys = movement_global_state_keys_iterator
			.iter()
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let mut count = 0;
		for movement_state_key in movement_global_state_keys {
			debug!("processing movement_state_key {}: {:?}", count, movement_state_key);

			let movement_state_key =
				movement_state_key.map_err(|e| CriterionError::Internal(e.into()))?;
			let movement_state_key_bytes = movement_state_key.encoded();
			let movement_aptos_state_key =
				MovementAptosStateKey::decode(movement_state_key_bytes.to_vec().as_slice())
					.map_err(|e| CriterionError::Internal(e.into()))?;

			let movement_value = movement_state_view
				.get_state_value_bytes(&movement_state_key)
				.map_err(|e| CriterionError::Internal(e.into()))?;

			match movement_value {
				Some(movement_value) => {
					let maptos_state_value = maptos_state_view
						.get_state_value_bytes(&movement_aptos_state_key)
						.map_err(|e| CriterionError::Internal(e.into()))?
						.ok_or(CriterionError::Unsatisfied(
							format!(
								"Movement Aptos is missing a value for {:?}",
								movement_state_key
							)
							.into(),
						))?;

					if movement_value != maptos_state_value {
						return Err(CriterionError::Unsatisfied(
							format!(
								"Movement state value for {:?} is {:?}, while Movement Aptos state value is {:?}",
								movement_state_key,
								movement_value,
								maptos_state_value
							)
							.into(),
						));
					}
				}
				None => {
					debug!("Value from a previous version has been removed at the latest ledger version");

					match maptos_state_view
						.get_state_value(&movement_aptos_state_key)
						.map_err(|e| CriterionError::Internal(e.into()))?
					{
						Some(_) => {
							return Err(CriterionError::Unsatisfied(
								format!(
									"Movement Aptos is unexpectedly not missing a value for {:?}",
									movement_state_key
								)
								.into(),
							));
						}
						None => {}
					}
				}
			}
			count += 1;
		}

		Ok(())
	}
}

impl Criterionish for GlobalStorageIncludes {
	fn satisfies(
		&self,
		movement_executor: &MovementExecutor,
		maptos_executor: &MovementAptosExecutor,
	) -> Result<(), CriterionError> {
		// get the latest ledger version from the movement executor
		let movement_ledger_version = movement_executor
			.latest_ledger_version()
			.map_err(|e| CriterionError::Internal(e.into()))?;

		for version in 0..=movement_ledger_version {
			self.satisfies_for_version(movement_executor, maptos_executor, version)
				.map_err(|e| CriterionError::Internal(e.into()))?;
		}

		Ok(())
	}
}
