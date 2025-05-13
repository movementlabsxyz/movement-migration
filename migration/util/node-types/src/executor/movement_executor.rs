use either::Either;
use maptos_opt_executor::aptos_storage_interface::state_view::DbStateView;
use maptos_opt_executor::aptos_storage_interface::DbReader;
use maptos_opt_executor::aptos_types::state_store::state_key::StateKey;
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
