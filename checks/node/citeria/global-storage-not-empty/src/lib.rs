use migration_node_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosNode, MovementNode,
};
pub struct GlobalStorageNotEmpty;

impl GlobalStorageNotEmpty {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl Criterionish for GlobalStorageNotEmpty {
	fn satisfies(
		&self,
		movement_executor: &MovementNode,
		maptos_executor: &MovementAptosNode,
	) -> Result<(), CriterionError> {
		// the movement state view is the domain, so the maptos state view is the codomain
		let movement_global_state_keys_iterator =
			movement_executor.global_state_keys_from_version(None);
		let movement_global_state_keys = movement_global_state_keys_iterator
			.iter()
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let count = movement_global_state_keys.count();
		if count == 0 {
			return Err(CriterionError::Unsatisfied(
				"Movement Aptos global storage is empty".into(),
			));
		}

		let maptos_global_state_keys_iterator =
			maptos_executor.global_state_keys_from_version(None);
		let maptos_global_state_keys = maptos_global_state_keys_iterator
			.iter()
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let count = maptos_global_state_keys.count();
		if count == 0 {
			return Err(CriterionError::Unsatisfied(
				"Movement Aptos global storage is empty".into(),
			));
		}

		Ok(())
	}
}
