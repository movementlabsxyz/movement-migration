pub use aptos_executor::block_executor::BlockExecutor as MovementAptosBlockExecutor;
use aptos_storage_interface::{
	state_store::state_view::db_state_view::{DbStateView, DbStateViewAtVersion},
	DbReader,
};
use aptos_types::state_store::state_key::StateKey;
pub use aptos_vm::aptos_vm::AptosVMBlockExecutor;
use either::Either;
use std::sync::Arc;

pub use aptos_executor::block_executor;
pub use aptos_types;
pub use aptos_types::state_store::TStateView;
use std::path::PathBuf;

use movement_aptos_core::Config as MovementAptosConfig;

/// The MovementAptos executor as would be presented in the criterion.
#[derive(Clone)]
pub struct MovementAptosNode {
	/// The db dir into which the aptos db was migrated.
	db_dir_path: PathBuf,

	/// The block executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	block_executor: Arc<MovementAptosBlockExecutor<AptosVMBlockExecutor>>,
}

impl MovementAptosNode {
	pub fn new(
		block_executor: MovementAptosBlockExecutor<AptosVMBlockExecutor>,
		db_dir_path: PathBuf,
	) -> Self {
		Self { block_executor: Arc::new(block_executor), db_dir_path }
	}

	/// Borrows the block executor.
	pub fn block_executor(&self) -> &MovementAptosBlockExecutor<AptosVMBlockExecutor> {
		&self.block_executor
	}

	/// Gets an [Arc] to the db reader.
	pub fn db_reader(&self) -> Arc<dyn DbReader> {
		self.block_executor().db.reader.clone()
	}

	/// Gets the db dir path
	pub fn db_dir_path(&self) -> &PathBuf {
		&self.db_dir_path
	}

	/// Gets the state view at a given version.
	pub fn state_view_at_version(
		&self,
		version: Option<u64>,
	) -> Result<DbStateView, anyhow::Error> {
		let state_view = self.db_reader().state_view_at_version(version)?;
		Ok(state_view)
	}

	/// Gets the all [StateKey]s in the global storage dating back to an original version. None is treated as 0 or all versions.
	pub fn global_state_keys_from_version(&self, version: Option<u64>) -> GlobalStateKeyIterable {
		GlobalStateKeyIterable { db_reader: self.db_reader(), version: version.unwrap_or(0) }
	}

	/// Forms a [MovementAptosConfig] with the given db dir path.
	pub fn test_movement_aptos_config(&self) -> Result<MovementAptosConfig, anyhow::Error> {
		Ok(MovementAptosConfig::test_node_config(self.db_dir_path())?)
	}
}

/// An iterable of [StateKey]s in the global storage dating back to an original version.
///
/// This helps deal with lifetime issues.
pub struct GlobalStateKeyIterable {
	db_reader: Arc<dyn DbReader>,
	version: u64,
}

const APTOS_MAX_WRITE_SET_SIZE: u64 = 20_000;

impl GlobalStateKeyIterable {
	pub fn iter(
		&self,
	) -> Result<Box<dyn Iterator<Item = Result<StateKey, anyhow::Error>> + '_>, anyhow::Error> {
		// Note: we may need to break out multiple iterators if the write set size is greater than the max.
		// This needs investigation.
		let write_set_iterator =
			self.db_reader.get_write_set_iterator(self.version, APTOS_MAX_WRITE_SET_SIZE)?;

		// We want to iterate lazily over the write set iterator because there could be a lot of them.
		let iter = write_set_iterator.flat_map(|res| match res {
			Ok(write_set) => {
				// It should be okay to collect because there should not be that many state keys in a write set.
				let items: Vec<_> = write_set.iter().map(|(key, _)| Ok(key.clone())).collect();
				Either::Left(items.into_iter())
			}
			Err(e) => Either::Right(std::iter::once(Err(e.into()))),
		});

		Ok(Box::new(iter))
	}
}
