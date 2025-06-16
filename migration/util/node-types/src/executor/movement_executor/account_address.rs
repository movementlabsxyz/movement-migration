use super::GlobalStateKeyIterable;
use super::{MovementNode, MovementOptExecutor};
use mtma_types::movement::aptos_types::account_address::AccountAddress;
use mtma_types::movement::aptos_types::state_store::state_key::inner::StateKeyInner;
use tracing::debug;

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

#[cfg(test)]
mod test {
	use super::*;
	use std::collections::HashSet;
	use tracing::info;

	#[tokio::test]
	#[tracing_test::traced_test]
	async fn test_iter_account_addresses() -> Result<(), anyhow::Error> {
		// form the executor
		let (movement_opt_executor, _temp_dir, _private_key, _receiver) =
			MovementOptExecutor::try_generated().await?;
		let movement_node = MovementNode::new(movement_opt_executor);

		// form the iterable
		let account_address_iterable = movement_node.iter_account_addresses(Some(0));
		let account_addresses = account_address_iterable.iter()?;

		let mut found_account_addresses = HashSet::new();
		for account_address in account_addresses {
			let account_address = account_address?;
			info!("Account address: {:?}", account_address);
			found_account_addresses.insert(account_address);
		}

		let comparison_account_addresses = HashSet::from([
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000001",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000002",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000003",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000004",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000005",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000006",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000007",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000008",
			)?,
			AccountAddress::from_hex_literal(
				"0x0000000000000000000000000000000000000000000000000000000000000009",
			)?,
			AccountAddress::from_hex_literal(
				"0x000000000000000000000000000000000000000000000000000000000000000a",
			)?,
			AccountAddress::from_hex_literal(
				"0x000000000000000000000000000000000000000000000000000000000a550c18",
			)?,
			AccountAddress::from_hex_literal(
				"0xd1126ce48bd65fb72190dbd9a6eaa65ba973f1e1664ac0cfba4db1d071fd0c36",
			)?,
		]);

		assert_eq!(found_account_addresses, comparison_account_addresses);

		Ok(())
	}
}
