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
		&self,
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
		.sender(self.0.address())
		.sequence_number(self.0.sequence_number())
		.max_gas_amount(5_000)
		.gas_unit_price(100);

		self.0.sign_with_transaction_builder(transaction_builder)
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

		// TODO: fetch the correct chain-id
		let chain_id = ChainId::test();

		let recipient1 = AccountAddress::random();
		let recipient2 = AccountAddress::random();

		// 1. transfer 1 MOVE to recipient1
		let tx1 = self.transfer(recipient1, 100_000_000, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx1).await.map_err(|e| {
			CriterionError::Unsatisfied(format!("Transfer to recipient1 failed: {:?}", e).into())
		})?;

		// 2. transfer 0 MOVE to recipient1
		let tx2 = self.transfer(recipient1, 0, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx2).await.map_err(|e| {
			CriterionError::Unsatisfied(
				format!("Zero transfer to recipient1 failed: {:?}", e).into(),
			)
		})?;

		// 3. transfers 0.1 MOVE to recipient2
		let tx3 = self.transfer(recipient2, 10_000_000, chain_id);

		movement_aptos_rest_client.submit_and_wait(&tx3).await.map_err(|e| {
			CriterionError::Unsatisfied(format!("Transfer to recipient2 failed: {:?}", e).into())
		})?;

		Ok(())
	}
}
