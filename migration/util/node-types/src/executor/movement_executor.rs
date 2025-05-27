use either::Either;
use maptos_opt_executor::aptos_crypto::HashValue;
use maptos_opt_executor::aptos_storage_interface::state_view::DbStateView;
use maptos_opt_executor::aptos_storage_interface::DbReader;
use maptos_opt_executor::aptos_types::state_store::state_key::StateKey;
use maptos_opt_executor::aptos_types::transaction::Version;
use maptos_opt_executor::aptos_types::{
	account_address::AccountAddress,
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	transaction::signature_verified_transaction::into_signature_verified_block,
	transaction::Transaction,
};
pub use maptos_opt_executor::Executor as MovementOptExecutor;
use movement_util::common_args::MovementArgs;
use std::sync::Arc;

use anyhow::Context;
pub use maptos_opt_executor;
pub use maptos_opt_executor::aptos_types::{chain_id::ChainId, state_store::TStateView};
use std::path::PathBuf;
use tracing::debug;
/// The Movement executor as would be presented in the criterion.
pub struct MovementNode {
	/// The opt executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	opt_executor: MovementOptExecutor,
}

impl MovementNode {
	pub fn new(opt_executor: MovementOptExecutor) -> Self {
		Self { opt_executor }
	}

	pub async fn from_dir(dir: PathBuf) -> Result<Self, anyhow::Error> {
		let movement_args = MovementArgs { movement_path: Some(dir.clone().display().to_string()) };

		let config = movement_args.config().await?;
		let maptos_config = config.execution_config.maptos_config;

		let (sender, _receiver) = futures_channel::mpsc::channel(1024);
		let opt_executor = MovementOptExecutor::try_from_config(maptos_config, sender).await?;

		Ok(Self::new(opt_executor))
	}

	/// Borrows the opt executor.
	pub fn opt_executor(&self) -> &MovementOptExecutor {
		&self.opt_executor
	}

	/// Borrows the opt executor mutably.
	pub fn opt_executor_mut(&mut self) -> &mut MovementOptExecutor {
		&mut self.opt_executor
	}

	/// Gets a clone of the chain id.
	pub fn chain_id(&self) -> ChainId {
		self.opt_executor().config().chain.maptos_chain_id.clone()
	}

	/// Gets the latest version of the ledger.
	pub fn latest_ledger_version(&self) -> Result<u64, anyhow::Error> {
		let db_reader = self.opt_executor().db_reader();

		let latest_ledger_info =
			db_reader.get_latest_ledger_info().context("failed to get latest ledger info")?;

		Ok(latest_ledger_info.ledger_info().version())
	}

	/// Constructs a [DbStateView] at a given version.
	pub fn state_view_at_version(
		&self,
		version: Option<u64>,
	) -> Result<DbStateView, anyhow::Error> {
		self.opt_executor().state_view_at_version(version)
	}

	/// Gets the all [StateKey]s in the global storage dating back to an original version. None is treated as 0 or all versions.
	pub fn global_state_keys_from_version(&self, version: Option<u64>) -> GlobalStateKeyIterable {
		GlobalStateKeyIterable {
			db_reader: self.opt_executor().db_reader(),
			version: version.unwrap_or(0),
		}
	}

	/// Gets the genesis block hash.
	pub fn genesis_block_hash(&self) -> Result<HashValue, anyhow::Error> {
		let db_reader = self.opt_executor().db_reader();
		let (_start, _end, block_event) = db_reader.get_block_info_by_version(0)?;
		Ok(block_event.hash()?)
	}

	/// Iterates over all blocks in the db.
	pub fn iter_blocks(&self, start_version: u64) -> Result<BlockIterator<'_>, anyhow::Error> {
		let latest_version = self.latest_ledger_version()?;
		Ok(BlockIterator { executor: self, version: start_version, latest_version })
	}

	/// Gets the genesis transaction.
	pub fn genesis_transaction(&self) -> Result<Transaction, anyhow::Error> {
		// get genesis transaction from db
		let db_reader = self.opt_executor().db_reader();
		let genesis_transaction =
			db_reader.get_transaction_by_version(0, self.latest_ledger_version()?, true)?;
		Ok(genesis_transaction.transaction)
	}

	/// Iterates over all transactions in the db.
	pub fn iter_transactions(
		&self,
		start_version: u64,
	) -> Result<TransactionIterator<'_>, anyhow::Error> {
		Ok(TransactionIterator::new(self, start_version, self.latest_ledger_version()?))
	}

	/// Iterates over all account keys
	/// NOTE: does this by checking all transactions and getting the [AccountAddress] signers from them
	pub fn iter_account_keys(
		&self,
		start_version: u64,
	) -> Result<AccountKeyIterator<'_>, anyhow::Error> {
		Ok(AccountKeyIterator::new(self, start_version))
	}
}

pub struct BlockIterator<'a> {
	executor: &'a MovementNode,
	version: u64,
	latest_version: u64,
}

impl<'a> Iterator for BlockIterator<'a> {
	type Item = Result<(Version, Version, ExecutableBlock), anyhow::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.version > self.latest_version {
			return None;
		}

		let db_reader = self.executor.opt_executor().db_reader();
		let (start_version, end_version, new_block_event) =
			match db_reader.get_block_info_by_version(self.version) {
				Ok(info) => info,
				Err(e) => return Some(Err(e.into())),
			};

		let mut transactions = Vec::new();
		for version in start_version..=end_version {
			let transaction =
				match db_reader.get_transaction_by_version(version, self.latest_version, false) {
					Ok(t) => t,
					Err(e) => return Some(Err(e.into())),
				};
			transactions.push(transaction.transaction);
		}

		let executable_transactions =
			ExecutableTransactions::Unsharded(into_signature_verified_block(transactions));
		let block = ExecutableBlock::new(
			match new_block_event.hash() {
				Ok(hash) => hash,
				Err(e) => return Some(Err(e.into())),
			},
			executable_transactions,
		);

		self.version = end_version + 1;
		Some(Ok((start_version, end_version, block)))
	}
}
/// An iterable of [StateKey]s in the global storage dating back to an original version.
///
/// This helps deal with lifetime issues.
pub struct GlobalStateKeyIterable {
	db_reader: Arc<dyn DbReader>,
	version: u64,
}

const MAX_WRITE_SET_SIZE: u64 = 20_000;

impl GlobalStateKeyIterable {
	pub fn iter(
		&self,
	) -> Result<Box<dyn Iterator<Item = Result<StateKey, anyhow::Error>> + '_>, anyhow::Error> {
		let write_set_iterator =
			self.db_reader.get_write_set_iterator(self.version, MAX_WRITE_SET_SIZE)?;

		// We want to iterate lazily over the write set iterator because there could be a lot of them.
		let mut count = 0;
		let iter = write_set_iterator.flat_map(move |res| match res {
			Ok(write_set) => {
				debug!("Iterating over write set {}", count);
				count += 1;
				// It should be okay to collect because there should not be that many state keys in a write set.
				let items: Vec<_> = write_set.iter().map(|(key, _)| Ok(key.clone())).collect();
				Either::Left(items.into_iter())
			}
			Err(e) => Either::Right(std::iter::once(Err(e.into()))),
		});

		Ok(Box::new(iter))
	}
}

pub struct TransactionIterator<'a> {
	executor: &'a MovementNode,
	version: u64,
	latest_version: u64,
	current_block_iter: Option<Box<dyn Iterator<Item = Transaction> + 'a>>,
}

impl<'a> TransactionIterator<'a> {
	fn new(executor: &'a MovementNode, version: u64, latest_version: u64) -> Self {
		Self { executor, version, latest_version, current_block_iter: None }
	}
}

impl<'a> Iterator for TransactionIterator<'a> {
	type Item = Result<Transaction, anyhow::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		// If we have a current block iterator, try to get the next transaction from it
		if let Some(iter) = &mut self.current_block_iter {
			if let Some(tx) = iter.next() {
				return Some(Ok(tx));
			}
			// If we've exhausted the current block's transactions, clear the iterator
			self.current_block_iter = None;
		}

		// If we've reached the end, we're done
		if self.version > self.latest_version {
			return None;
		}

		// Get the next block
		let mut block_iterator = match self.executor.iter_blocks(self.version) {
			Ok(iter) => iter,
			Err(e) => return Some(Err(e)),
		};

		// Get the next block and update version
		let (_, end_version, block) = match block_iterator.next() {
			Some(Ok(block_info)) => block_info,
			Some(Err(e)) => return Some(Err(e)),
			None => return None,
		};
		self.version = end_version + 1;

		// Create an iterator for this block's transactions
		self.current_block_iter =
			Some(Box::new(block.transactions.into_txns().into_iter().map(|tx| tx.into_inner())));

		// Recursively call next to get the first transaction from the new block
		self.next()
	}
}

pub struct AccountKeyIterator<'a> {
	executor: &'a MovementNode,
	version: u64,
	latest_version: u64,
	current_tx_iter: Option<Box<dyn Iterator<Item = Result<AccountAddress, anyhow::Error>> + 'a>>,
}

impl<'a> AccountKeyIterator<'a> {
	fn new(executor: &'a MovementNode, start_version: u64) -> Self {
		Self {
			executor,
			version: start_version,
			latest_version: executor.latest_ledger_version().unwrap_or(u64::MAX),
			current_tx_iter: None,
		}
	}

	fn get_next_address(&mut self) -> Option<Result<AccountAddress, anyhow::Error>> {
		// If we don't have a transaction iterator, get one
		if self.current_tx_iter.is_none() {
			match self.executor.iter_transactions(self.version) {
				Ok(iter) => {
					// Create an iterator that extracts account addresses from transactions
					let addresses_iter = iter.flat_map(|tx_result| {
						match tx_result {
							Ok(tx) => {
								// Extract account address from transaction
								match <maptos_opt_executor::aptos_types::transaction::Transaction as TryInto<Transaction>>::try_into(tx) {
									Ok(t) => {
										if let Ok(user_tx) = maptos_opt_executor::aptos_types::transaction::SignedTransaction::try_from(t) {
											vec![Ok(user_tx.sender())]
										} else {
											Vec::new() // Skip non-user transactions
										}
									}
									_ => Vec::new(), // Skip non-user transactions
								}
							}
							Err(e) => vec![Err(e)],
						}
					});
					self.current_tx_iter = Some(Box::new(addresses_iter));
				}
				Err(e) => return Some(Err(e)),
			}
		}

		// Try to get the next address from our iterator
		if let Some(iter) = &mut self.current_tx_iter {
			if let Some(addr_result) = iter.next() {
				return Some(addr_result);
			}
		}

		// If we've exhausted the current iterator, move to next version
		self.current_tx_iter = None;
		self.version += 1;
		if self.version <= self.latest_version {
			self.get_next_address()
		} else {
			None
		}
	}
}

impl<'a> Iterator for AccountKeyIterator<'a> {
	type Item = Result<AccountAddress, anyhow::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self.get_next_address()
	}
}
