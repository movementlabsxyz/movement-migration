use anyhow::Context;
use aptos_sdk::move_types::{
	identifier::Identifier, language_storage::ModuleId, language_storage::TypeTag,
};
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::{
	account_address::AccountAddress,
	chain_id::ChainId,
	transaction::{EntryFunction, SignedTransaction, TransactionPayload},
	LocalAccount,
};
use mtma_migrator_test_types::criterion::{
	Criterion, CriterionError, Criterionish, MovementAptosMigrator, MovementMigrator,
};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MaptosTransferLifecycle(LocalAccount);

impl MaptosTransferLifecycle {
	pub fn new(local_account: LocalAccount) -> Self {
		Self(local_account)
	}

	pub fn criterion(local_account: LocalAccount) -> Criterion<Self> {
		Criterion::new(Self(local_account))
	}

	pub fn transfer(
		from_account: &LocalAccount,
		to_account: AccountAddress,
		amount: u64,
		chain_id: ChainId,
	) -> SignedTransaction {
		let transaction_builder = TransactionBuilder::new(
			TransactionPayload::EntryFunction(EntryFunction::new(
				ModuleId::new(AccountAddress::ONE, Identifier::new("aptos_coin").unwrap()),
				Identifier::new("transfer").unwrap(),
				vec![TypeTag::from_str("0x1::aptos_coin::AptosCoin").unwrap()],
				vec![bcs::to_bytes(&to_account).unwrap(), bcs::to_bytes(&amount).unwrap()],
			)),
			SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 10,
			chain_id,
		)
		.sender(from_account.address())
		.sequence_number(from_account.sequence_number())
		.max_gas_amount(5_000)
		.gas_unit_price(100);

		from_account.sign_with_transaction_builder(transaction_builder)
	}
}

impl Criterionish for MaptosTransferLifecycle {
	async fn satisfies(
		&self,
		_movement_migrator: &MovementMigrator,
		movement_aptos_migrator: &MovementAptosMigrator,
	) -> Result<(), CriterionError> {
		let movement_aptos_rest_client = movement_aptos_migrator
			.wait_for_rest_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let movement_aptos_faucet_client = movement_aptos_migrator
			.wait_for_faucet_client_ready(tokio::time::Duration::from_secs(30))
			.await
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let alice = LocalAccount::generate(&mut rand::rngs::OsRng);
		let bob = LocalAccount::generate(&mut rand::rngs::OsRng);

		movement_aptos_faucet_client
			.create_account(alice.address())
			.await
			.context("Failed to create Alice's account")
			.map_err(|e| CriterionError::Internal(e.into()))?;

		movement_aptos_faucet_client
			.create_account(bob.address())
			.await
			.context("Failed to create Bob's account")
			.map_err(|e| CriterionError::Internal(e.into()))?;

		let chain_id = movement_aptos_rest_client
			.get_index()
			.await
			.context("Failed to get chain ID")
			.map_err(|e| CriterionError::Internal(e.into()))?
			.inner()
			.chain_id;
		let chain_id = ChainId::new(chain_id);

		// 1. transfer 1 MOVE to Alice
		let tx1 =
			MaptosTransferLifecycle::transfer(&self.0, alice.address(), 100_000_000, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx1).await.map_err(|e| {
			CriterionError::Unsatisfied(format!("Transfer to Aice failed: {:?}", e).into())
		})?;

		// 2. transfer 0 MOVE to Alice
		let tx2 = MaptosTransferLifecycle::transfer(&alice, alice.address(), 0, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx2).await.map_err(|e| {
			CriterionError::Unsatisfied(
				format!("Zero transfer from Alice to Alice failed: {:?}", e).into(),
			)
		})?;

		// 3. transfers 0.1 MOVE to Bob
		let tx3 = MaptosTransferLifecycle::transfer(&alice, bob.address(), 10_000_000, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx3).await.map_err(|e| {
			CriterionError::Unsatisfied(format!("Transfer to Bob failed: {:?}", e).into())
		})?;

		Ok(())
	}
}
