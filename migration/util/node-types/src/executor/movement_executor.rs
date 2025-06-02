use anyhow::Context;
use either::Either;
pub use maptos_opt_executor::Executor as MovementOptExecutor;
use movement_util::common_args::MovementArgs;
use mtma_types::movement::aptos_crypto::HashValue;
use mtma_types::movement::aptos_storage_interface::state_view::DbStateView;
use mtma_types::movement::aptos_storage_interface::DbReader;
use mtma_types::movement::aptos_types::state_store::state_key::StateKey;
use mtma_types::movement::aptos_types::transaction::Version;
use mtma_types::movement::aptos_types::{
	account_address::AccountAddress,
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	transaction::signature_verified_transaction::into_signature_verified_block,
	transaction::Transaction,
};
pub use mtma_types::movement::aptos_types::{chain_id::ChainId, state_store::TStateView};
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::info;

pub use maptos_opt_executor;
use tracing::debug;
use uuid::Uuid;
use walkdir::WalkDir;

/// The Movement executor as would be presented in the criterion.
pub struct MovementNode {
	/// The opt executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	opt_executor: MovementOptExecutor,
}

/// Copies a directory recursively while ignoring specified paths
fn copy_dir_recursive_with_ignore(src: &Path, ignore_paths: impl IntoIterator<Item = impl AsRef<Path>>, dst: &Path) -> Result<(), anyhow::Error> {
    let ignore_paths: Vec<_> = ignore_paths.into_iter().collect();
    let mut errors: Vec<Result<(), anyhow::Error>> = Vec::new();

    // Configure WalkDir to be more resilient
    let walker = WalkDir::new(src)
        .follow_links(false)
        .same_file_system(true)
        .into_iter()
        .filter_entry(|entry| {
            // Early filter for ignored paths to avoid permission errors
            let path = entry.path();
            !ignore_paths.iter().any(|ignore| path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref()))
        });

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                // Check if this is a permission error on an ignored path
                if let Some(path) = e.path() {
                    if ignore_paths.iter().any(|ignore| path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref())) {
                        debug!("Ignoring permission error on ignored path: {}", path.display());
                        continue;
                    }
                }
                // For other errors, log with info for now and fail.
                info!("Error accessing path: {:?}", e);
				errors.push(Err(anyhow::anyhow!("failed to get entry: {:?}", e)));
                continue;
            }
        };

        debug!("Processing entry: {}", entry.path().display());

        let path = entry.path();
        let dest_path = dst.join(path.strip_prefix(src).context("failed to strip prefix")?);

        if entry.file_type().is_dir() {
            match fs::create_dir_all(&dest_path) {
                Ok(_) => (),
                Err(e) => errors.push(Err(e.into())),
            }
        } else {
            if let Some(parent) = dest_path.parent() {
                match fs::create_dir_all(parent) {
                    Ok(_) => (),
                    Err(e) => errors.push(Err(e.into())),
                }
            }
            match fs::copy(path, &dest_path) {
                Ok(_) => (),
                Err(e) => errors.push(Err(e.into())),
            }
        }
    }

    // Combine all results into one
    if !errors.is_empty() {
        let mut total_error_message = String::from("failed to copy directory with the following errors: ");
        for error in errors {
            total_error_message = total_error_message + &format!("{:?}\n", error);
        }
        return Err(anyhow::anyhow!(total_error_message));
    }

    Ok(())
}

/// Sets all permission in a directory recursively.
fn set_permissions_recursive(path: &Path, permissions: Permissions) -> std::io::Result<()> {
	for entry in WalkDir::new(path) {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			fs::set_permissions(&path, permissions.clone())?;
		}
	}
	Ok(())
}

impl MovementNode {
	pub fn new(opt_executor: MovementOptExecutor) -> Self {
		Self { opt_executor }
	}

	pub async fn try_from_dir(dir: PathBuf) -> Result<Self, anyhow::Error> {
		// copy the dir to a new .debug dir
		let uuid = Uuid::new_v4().to_string();
		let debug_dir = PathBuf::from(format!(".debug/{}", uuid));

		// Copy the entire .movement directory recursively
		let movement_dir = dir.join(".movement");

		// set the permissions on the movement dir to 755
		// Note: this would mess up celestia node permissions, but we don't care about that here.
		// We really only care about maptos db permissions.
		fs::set_permissions(&movement_dir, Permissions::from_mode(0o755)).context(format!("failed to set permissions on the movement directory {}", movement_dir.display()))?;

		// don't copy anything from the celestia directory
		copy_dir_recursive_with_ignore(&movement_dir, [PathBuf::from("celestia")], &debug_dir).context("failed to copy movement dir")?;

		// Set all permissions in the debug directory recursively
		// Note: this would mess up celestia node permissions, but we don't care about that here.
		// We really only care about maptos db permissions.
		// TODO: tighten the copying accordingly.
		set_permissions_recursive(&debug_dir, Permissions::from_mode(0o755)).context(format!("failed to set permissions on the debug directory {}", debug_dir.display()))?;

		let movement_args = MovementArgs { movement_path: Some(debug_dir.display().to_string()) };

		let config = movement_args.config().await.context("failed to get movement config")?;
		let old_db_path = config
			.execution_config
			.maptos_config
			.chain
			.maptos_db_path
			.clone()
			.context("failed to get old db path")?;

		// check, old db path should begin with /.movement since it is based on a docker volume
		if !old_db_path.starts_with("/.movement") {
			return Err(anyhow::anyhow!(
				"old db path must begin with /.movement since it is based on a docker volume"
			));
		}

		let mut maptos_config = config.execution_config.maptos_config;

		// Get the relative path from the old directory to the db path
		let rel_path = old_db_path
			.strip_prefix("/.movement")
			.context("db path must be within the movement directory")?;

		// Create new db path by joining debug dir with the relative path
		let new_db_path = debug_dir.join(".movement").join(rel_path);
		maptos_config.chain.maptos_db_path =
			Some(PathBuf::from(new_db_path.to_string_lossy().into_owned()));

		info!("maptos config: {:?}", maptos_config);

		let (sender, _receiver) = futures_channel::mpsc::channel(1024);
		let opt_executor = MovementOptExecutor::try_from_config(maptos_config, sender)
			.await
			.context("failed to create movement opt executor")?;

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
	pub fn iter_account_addresses(
		&self,
		start_version: u64,
	) -> Result<AccountAddressIterator<'_>, anyhow::Error> {
		Ok(AccountAddressIterator::new(self, start_version))
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
		info!("Getting next transaction from block iterator");
		if let Some(iter) = &mut self.current_block_iter {
			if let Some(tx) = iter.next() {
				info!("Got next transaction from block iterator {:?}", tx);
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

pub struct AccountAddressIterator<'a> {
	executor: &'a MovementNode,
	version: u64,
	latest_version: u64,
	current_tx_iter: Option<Box<dyn Iterator<Item = Result<AccountAddress, anyhow::Error>> + 'a>>,
}

impl<'a> AccountAddressIterator<'a> {
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
		info!("Getting next address from transaction iterator");
		if self.current_tx_iter.is_none() {
			match self.executor.iter_transactions(self.version) {
				Ok(iter) => {
					// Create an iterator that extracts account addresses from transactions
					info!("Creating iterator for transactions");
					let addresses_iter = iter.flat_map(|tx_result| {
						match tx_result {
							Ok(tx) => {
								// Extract account address from transaction
								match <mtma_types::movement::aptos_types::transaction::Transaction as TryInto<Transaction>>::try_into(tx) {
									Ok(t) => {
										if let Ok(user_tx) = mtma_types::movement::aptos_types::transaction::SignedTransaction::try_from(t) {
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
		info!("Trying to get next address from iterator");
		if let Some(iter) = &mut self.current_tx_iter {
			if let Some(addr_result) = iter.next() {
				return Some(addr_result);
			}
		}

		// If we've exhausted the current iterator, move to next version
		info!("Exhausted current iterator, moving to next version");
		self.current_tx_iter = None;
		self.version += 1;
		if self.version <= self.latest_version {
			self.get_next_address()
		} else {
			None
		}
	}
}

impl<'a> Iterator for AccountAddressIterator<'a> {
	type Item = Result<AccountAddress, anyhow::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		info!("Getting next address from account address iterator");
		self.get_next_address()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use tempfile::TempDir;

	#[test]
	#[tracing_test::traced_test]
	fn test_copy_dir_recursive_with_ignore() -> Result<(), anyhow::Error> {
		// put some file in a temp dir
		let temp_dir = TempDir::new()?;

		// write a file that should be copied
		let file_path = temp_dir.path().join("maptos").join("test_file.txt");
		fs::create_dir_all(file_path.parent().context("failed to get parent directory for file that should be copied")?).context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should be copied")?;

		// write a file that should not be copied
		let file_path = temp_dir.path().join("celestia").join("test_file2.txt");
		fs::create_dir_all(file_path.parent().context("failed to get parent directory for file that should not be copied")?).context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should not be copied")?;

		// create the target temp dir
		let dst = TempDir::new()?;

		// copy the file to a new dir, ignoring celestia directory
		copy_dir_recursive_with_ignore(&temp_dir.path(), [PathBuf::from("celestia")], &dst.path()).context("failed to copy directory")?;

		// check that the file was copied
		assert!(dst.path().join("maptos").join("test_file.txt").exists());

		// check that the celestia directory was not copied
		assert!(!dst.path().join("celestia").join("test_file2.txt").exists());

		Ok(())
	}

	// Somehow the following test is failing on CI: https://github.com/movementlabsxyz/movement-migration/actions/runs/15386989846/job/43287594343
	// This indicates that failure is not due to the inability to ignore copy, but rather some issue performing an oepration that requires permissions.
	#[test]
	fn test_are_you_kidding_me() -> Result<(), anyhow::Error> {

		let source_dir = TempDir::new()?;
		let target_dir = TempDir::new()?;

		let path_that_must_be_ignored = source_dir.path().join(".movement/celestia/c1860ae680eb2d91927b/.celestia-app/keyring-test");

		fs::create_dir_all(path_that_must_be_ignored.parent().context("failed to get parent directory for path that must be ignored")?).context("failed to create directory")?;
		// write a file that must not be ignored
		fs::write(path_that_must_be_ignored.clone(), "test").context("failed to write file that must not be ignored")?;
		// set permissions to 000 on the file and then on the parent directory
		fs::set_permissions(path_that_must_be_ignored.clone(), Permissions::from_mode(0o000)).context("failed to set permissions on file that must not be ignored")?;
		fs::set_permissions(path_that_must_be_ignored.parent().context("failed to get parent directory for path that must be ignored")?, Permissions::from_mode(0o000)).context("failed to set permissions on parent directory that must not be ignored")?;

		copy_dir_recursive_with_ignore(source_dir.path(), ["celestia"], target_dir.path()).context("failed to copy directory")?;

		assert!(!target_dir.path().join("celestia").join("c1860ae680eb2d91927b").join(".celestia-app").join("keyring-test").exists());

		Ok(())
	}

}