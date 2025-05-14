use crate::criterion::CriterionError;
use aptos_rest_client::Client as MovementAptosRestClient;
use aptos_sdk::types::{transaction::EntryFunction, LocalAccount};
pub use aptos_sdk::*;
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

	/// Simulates a script function.
	pub fn simulate_script(
		&self,
		_signer: &mut LocalAccount,
		_script_code: &[u8],
		_arguments: Vec<Vec<u8>>,
	) -> Result<(), CriterionError> {
		unimplemented!()
	}

	/// Simulates an entry function.
	pub fn simulate_entry_function(
		&self,
		_signer: &mut LocalAccount,
		_entry_function: &EntryFunction,
		_arguments: Vec<Vec<u8>>,
	) -> Result<(), CriterionError> {
		unimplemented!()
	}

	/// Checks if a feature is enabled.
	pub fn check_feature_enabled(&self, _feature_id: u64) -> Result<bool, CriterionError> {
		unimplemented!()
	}
}
