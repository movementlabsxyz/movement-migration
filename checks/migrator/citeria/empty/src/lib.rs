use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
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
	async fn satisfies(
		&self,
		_movement_migrator: &MovementMigrator,
		_movement_aptos_migrator: &MovementAptosMigrator,
	) -> Result<(), CriterionError> {
		Ok(())
	}
}
