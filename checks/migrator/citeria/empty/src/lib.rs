use mtma_node_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosNode, MovementNode,
};

pub struct Empty;

impl Empty {
	pub fn new() -> Self {
		Self
	}

	pub fn criterion() -> Criterion<Self> {
		Criterion::new(Self)
	}
}

impl Criterionish for Empty {
	fn satisfies(
		&self,
		_movement_executor: &MovementNode,
		_movement_aptos_executor: &MovementAptosNode,
	) -> Result<(), CriterionError> {
		Ok(())
	}
}
