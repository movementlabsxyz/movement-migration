use super::GlobalStateKeyIterable;
use super::MovementNode;
use mtma_types::movement::aptos_types::account_address::AccountAddress;
use mtma_types::movement::aptos_types::state_store::state_key::inner::StateKeyInner;
use tracing::{debug, info};

impl MovementNode {
	/// Iterates over all account keys
	/// NOTE: does this by checking the [AccessPath]s on all global storage keys.
	pub fn iter_account_addresses(&self, version: Option<u64>) -> AccountAddressIterable {
		AccountAddressIterable {
			global_state_key_iterable: self.global_state_keys_from_version(version),
		}
	}
}

/// An iterable of [AccountAddress]es extracted from global storage keys dating back to an original version.
pub struct AccountAddressIterable {
	global_state_key_iterable: GlobalStateKeyIterable,
}

impl AccountAddressIterable {
	pub fn iter(
		&self,
	) -> Result<Box<dyn Iterator<Item = Result<AccountAddress, anyhow::Error>> + '_>, anyhow::Error>
	{
		let state_key_iter = self.global_state_key_iterable.iter()?;

		// Map over StateKey to extract AccountAddress using AccessPath
		let addresses_iter = state_key_iter.flat_map(|state_key_result| match state_key_result {
			Ok(state_key) => match state_key.inner() {
				StateKeyInner::AccessPath(access_path) => {
					debug!("Flat mapping state key to account address: {:?}", state_key);
					vec![Ok(access_path.address)]
				}
				_ => vec![],
			},
			Err(e) => vec![Err(e)],
		});

		Ok(Box::new(addresses_iter))
	}
}
