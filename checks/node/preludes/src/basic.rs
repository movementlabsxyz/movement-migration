use migration_node_test_types::{
	criterion::movement_executor::maptos_opt_executor::{
		aptos_crypto::ed25519::Ed25519PrivateKey,
		aptos_sdk::{
			transaction_builder::TransactionFactory,
			types::{AccountKey, LocalAccount},
		},
		aptos_types::{account_config::aptos_test_root_address, chain_id::ChainId},
	},
	prelude::{Prelude, PreludeError, PreludeGenerator},
};
use rand::SeedableRng;

/// A prelude that creates a new account and mints some coins to it.
pub struct BasicPrelude {
	pub private_key: Ed25519PrivateKey,
	pub chain_id: ChainId,
}

impl BasicPrelude {
	/// Generate a prelude with a new account and minted coins.
	pub async fn generate(
		private_key: Ed25519PrivateKey,
		chain_id: ChainId,
	) -> Result<Prelude, PreludeError> {
		let prelude = BasicPrelude { private_key, chain_id }.generate().await?;
		Ok(prelude)
	}
}

/// A prelude that creates a new account and mints some coins to it.
impl PreludeGenerator for BasicPrelude {
	async fn generate(self) -> Result<Prelude, PreludeError> {
		let seed = [3u8; 32];
		let mut rng = ::rand::rngs::StdRng::from_seed(seed);

		// form the prelude
		let mut prelude = Prelude::new_empty();

		// add some transactions
		let root_account = LocalAccount::new(
			aptos_test_root_address(),
			AccountKey::from_private_key(self.private_key),
			0,
		);
		let current_time_microseconds = chrono::Utc::now().timestamp_micros() as u64;
		let tx_factory = TransactionFactory::new(self.chain_id)
			.with_transaction_expiration_time(current_time_microseconds + 60_000_000);

		// create a new account
		let new_account = LocalAccount::generate(&mut rng);
		let user_account_creation_tx = root_account.sign_with_transaction_builder(
			tx_factory.create_user_account(new_account.public_key()),
		);
		prelude.add_transaction(user_account_creation_tx);

		// mint some coins to the new account
		let mint_tx = root_account
			.sign_with_transaction_builder(tx_factory.mint(new_account.address(), 2000));
		prelude.add_transaction(mint_tx);

		Ok(prelude)
	}
}
