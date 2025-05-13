use either::Either;
use maptos_opt_executor::aptos_crypto::HashValue;
use maptos_opt_executor::aptos_storage_interface::state_view::DbStateView;
use maptos_opt_executor::aptos_storage_interface::DbReader;
use maptos_opt_executor::aptos_types::state_store::state_key::StateKey;
use maptos_opt_executor::aptos_types::transaction::Version;
use maptos_opt_executor::aptos_types::{
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	transaction::signature_verified_transaction::into_signature_verified_block,
	transaction::Transaction,
};
pub use maptos_opt_executor::Executor as MovementOptExecutor;
use std::sync::Arc;

use anyhow::Context;
pub use maptos_opt_executor;
pub use maptos_opt_executor::aptos_types::{chain_id::ChainId, state_store::TStateView};
use tracing::debug;
/// The Movement executor as would be presented in the criterion.
pub struct MovementExecutor {
	/// The opt executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	opt_executor: MovementOptExecutor,
}

impl MovementExecutor {
	pub fn new(opt_executor: MovementOptExecutor) -> Self {
		Self { opt_executor }
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
	pub fn iter_blocks(&self, version: u64) -> Result<BlockIterator<'_>, anyhow::Error> {
		let latest_version = self.latest_ledger_version()?;
		println!("latest_version: {}", latest_version);
		Ok(BlockIterator { executor: self, version, latest_version })
	}

	/// Gets the genesis transaction.
	pub fn genesis_transaction(&self) -> Result<Transaction, anyhow::Error> {
		// get genesis transaction from db
		let db_reader = self.opt_executor().db_reader();
		let genesis_transaction =
			db_reader.get_transaction_by_version(0, self.latest_ledger_version()?, true)?;
		Ok(genesis_transaction.transaction)
	}
}

pub struct BlockIterator<'a> {
	executor: &'a MovementExecutor,
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
