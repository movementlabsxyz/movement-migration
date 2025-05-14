pub mod movement_aptos_migrator_client;
pub mod movement_migrator_client;
pub use movement_aptos_migrator_client::MovementAptosMigratorClient;
pub use movement_migrator_client::MovementMigratorClient;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("failed to build from config: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("encountered an error while checking the criterion: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub trait Criterionish {
	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	fn satisfies(
		&self,
		movement_e2e_client: &MovementMigratorClient,
		movement_aptos_e2e_client: &MovementAptosMigratorClient,
	) -> Result<(), CriterionError>;
}

/// The criterion type simply
pub struct Criterion<T>(T)
where
	T: Criterionish;

impl<T> Criterion<T>
where
	T: Criterionish,
{
	pub fn new(t: T) -> Self {
		Self(t)
	}

	/// Whether the criterion is satisfied by the given movement and movement_aptos executors.
	pub fn satisfies(
		&self,
		movement_e2e_client: &MovementMigratorClient,
		movement_aptos_e2e_client: &MovementAptosMigratorClient,
	) -> Result<(), CriterionError> {
		self.0.satisfies(movement_e2e_client, movement_aptos_e2e_client)
	}
}
