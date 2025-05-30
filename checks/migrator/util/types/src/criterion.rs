use mtma_types::movement::movement_client::rest_client::Client as MovementRestClient;
use mtma_types::movement_aptos::aptos_rest_client::Client as MovementAptosRestClient;

/// The MovementAptos executor as would be presented in the criterion.
#[derive(Debug)]
pub struct MovementAptosMigratorClient {
	/// The rest client.
	rest_client: MovementAptosRestClient,
}

impl MovementAptosMigratorClient {
	pub fn new(rest_client: MovementAptosRestClient) -> Self {
		Self { rest_client }
	}

	/// Borrows the block executor.
	pub fn rest_client(&self) -> &MovementAptosRestClient {
		&self.rest_client
	}
}

/// The Movement executor as would be presented in the criterion.
#[derive(Debug)]
pub struct MovementMigratorClient {
	/// The rest client.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	rest_client: MovementRestClient,
}

impl MovementMigratorClient {
	pub fn new(rest_client: MovementRestClient) -> Self {
		Self { rest_client }
	}

	/// Borrows the opt executor.
	pub fn rest_client(&self) -> &MovementRestClient {
		&self.rest_client
	}
}

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum CriterionError {
	#[error("criterion was not satisfied: {0}")]
	Unsatisfied(#[source] Box<dyn std::error::Error + Send + Sync>),
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
