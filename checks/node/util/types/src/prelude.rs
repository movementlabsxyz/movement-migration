use crate::criterion::MovementNode;
use mtma_types::movement::aptos_crypto::HashValue;
use mtma_types::movement::aptos_types::{
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	block_metadata::BlockMetadata,
	transaction::signature_verified_transaction::into_signature_verified_block,
	transaction::SignedTransaction,
	transaction::Transaction,
};
use std::future::Future;

const DEFAULT_INCREMENT_MICROS: u64 = 1_000_000;
const DEFAULT_EPOCH: u64 = 0;
const DEFAULT_INCREMENT_EPOCH_EACH: u64 = 100;
const DEFAULT_ROUND: u64 = 0;

/// Errors thrown when working with the [Config].
#[derive(Debug, thiserror::Error)]
pub enum PreludeError {
	#[error("prelude encountered an internal error: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A prelude is several blocks of transactions represented as Vec<Vec<SignedTransaction>>.
#[derive(Debug, Clone)]
pub struct Prelude(Vec<Vec<SignedTransaction>>);

impl Prelude {
	pub fn new_empty() -> Self {
		Self(Vec::new())
	}

	/// Adds a transaction to the current block.
	pub fn add_transaction(&mut self, transaction: SignedTransaction) -> &mut Self {
		// push to the last or create one if there isn't one
		if self.0.is_empty() {
			self.0.push(Vec::new());
		}

		self.0.last_mut().unwrap().push(transaction);

		self
	}

	/// Ends the current block and starts a new one.
	pub fn end_block(&mut self) -> &mut Self {
		self.0.push(Vec::new());

		self
	}

	pub fn new(
		transactions: impl IntoIterator<Item = impl IntoIterator<Item = SignedTransaction>>,
	) -> Self {
		Self(transactions.into_iter().map(|t| t.into_iter().collect()).collect())
	}

	/// Gets executable blocks from the prelude.
	///
	/// # Parameters
	/// * `executor` - The movement executor to use
	/// * `epoch` - The epoch to start with
	/// * `increment_epoch_each` - The number of blocks before incrementing the epoch
	/// * `round` - The round to start with
	/// * `current_timestamp_micros` - The current timestamp in microseconds
	/// * `timestamp_increment_micros` - The number of microseconds to increment the timestamp by
	///
	/// TODO: these parameters could be better as a builder struct or something.
	pub fn get_executable_blocks(
		&self,
		executor: &MovementNode,
		mut epoch: u64,
		increment_epoch_each: u64,
		mut round: u64,
		mut current_timestamp_micros: u64,
		timestamp_increment_micros: u64,
	) -> Result<Vec<ExecutableBlock>, PreludeError> {
		let mut executable_blocks = Vec::new();

		for signed_transactions in &self.0 {
			// block id is always random, we're the proposer
			let block_id = HashValue::random();

			let block_metadata = Transaction::BlockMetadata(BlockMetadata::new(
				block_id.clone(),
				epoch,
				round,
				executor.opt_executor().signer.author(),
				vec![],
				vec![],
				current_timestamp_micros,
			));

			let mut transactions = vec![block_metadata];
			transactions.extend(
				signed_transactions.into_iter().map(|t| Transaction::UserTransaction(t.clone())),
			);

			let executable_transactions =
				ExecutableTransactions::Unsharded(into_signature_verified_block(transactions));

			let block = ExecutableBlock::new(block_id, executable_transactions);

			executable_blocks.push(block);

			round += 1;
			current_timestamp_micros += timestamp_increment_micros;

			if round % increment_epoch_each == increment_epoch_each - 1 {
				epoch += 1;
			}
		}

		Ok(executable_blocks)
	}

	pub async fn run(&self, movement_executor: &mut MovementNode) -> Result<(), PreludeError> {
		let current_timestamp_micros = chrono::Utc::now().timestamp_micros() as u64;

		let executable_blocks = self
			.get_executable_blocks(
				movement_executor,
				DEFAULT_EPOCH,
				DEFAULT_INCREMENT_EPOCH_EACH,
				DEFAULT_ROUND,
				current_timestamp_micros,
				DEFAULT_INCREMENT_MICROS,
			)
			.map_err(|e| PreludeError::Internal(e.into()))?;

		for block in executable_blocks {
			movement_executor
				.opt_executor_mut()
				.execute_block(block)
				.map_err(|e| PreludeError::Internal(e.into()))?;
		}

		Ok(())
	}
}

pub trait PreludeGenerator {
	/// Generates a prelude.
	///
	/// We'll make this async for now because we may want to pull some things off the wire at some point.
	fn generate(self) -> impl Future<Output = Result<Prelude, PreludeError>>;
}
