use anyhow::Context;

pub use maptos_opt_executor::Executor as MovementOptExecutor;
use movement_util::common_args::MovementArgs;
use mtma_types::movement::aptos_crypto::HashValue;
use mtma_types::movement::aptos_storage_interface::state_view::DbStateView;
use mtma_types::movement::aptos_types::state_store::state_key::StateKey;
use mtma_types::movement::aptos_types::transaction::Transaction;
pub use mtma_types::movement::aptos_types::{chain_id::ChainId, state_store::TStateView};
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::info;

pub use maptos_opt_executor;
use tracing::debug;
use uuid::Uuid;
use walkdir::WalkDir;

use bytes::Bytes;

pub mod account_address;
pub use account_address::*;
pub mod block;
pub use block::*;
pub mod transaction;
pub use transaction::*;
pub mod global_state_key;
pub use global_state_key::*;

/// The Movement executor as would be presented in the criterion.
pub struct MovementNode {
	/// The opt executor.
	///
	/// We will have this remain private because I don't think we want people mutating it in the criterion.
	opt_executor: MovementOptExecutor,
}

/// Copies a directory recursively while ignoring specified paths
fn copy_dir_recursive_with_ignore(
	src: &Path,
	ignore_paths: impl IntoIterator<Item = impl AsRef<Path>>,
	dst: &Path,
) -> Result<(), anyhow::Error> {
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
			!ignore_paths.iter().any(|ignore| {
				path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref())
			})
		});

	for entry in walker {
		let entry = match entry {
			Ok(e) => e,
			Err(e) => {
				// Check if this is a permission error on an ignored path
				if let Some(path) = e.path() {
					if ignore_paths.iter().any(|ignore| {
						path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref())
					}) {
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
		let mut total_error_message =
			String::from("failed to copy directory with the following errors: ");
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
		fs::set_permissions(&movement_dir, Permissions::from_mode(0o755)).context(format!(
			"failed to set permissions on the movement directory {}",
			movement_dir.display()
		))?;

		// don't copy anything from the celestia directory
		copy_dir_recursive_with_ignore(&movement_dir, [PathBuf::from("celestia")], &debug_dir)
			.context("failed to copy movement dir")?;

		// Set all permissions in the debug directory recursively
		// Note: this would mess up celestia node permissions, but we don't care about that here.
		// We really only care about maptos db permissions.
		// TODO: tighten the copying accordingly.
		set_permissions_recursive(&debug_dir, Permissions::from_mode(0o755)).context(format!(
			"failed to set permissions on the debug directory {}",
			debug_dir.display()
		))?;

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

	/// Get state value bytes for a given version and state key.
	pub fn get_state_value_bytes(
		&self,
		version: Option<u64>,
		state_key: StateKey,
	) -> Result<Option<Bytes>, anyhow::Error> {
		let state_view = self.state_view_at_version(version)?;
		let bytes = state_view.get_state_value_bytes(&state_key)?;
		Ok(bytes)
	}

	/// Gets the genesis block hash.
	pub fn genesis_block_hash(&self) -> Result<HashValue, anyhow::Error> {
		let db_reader = self.opt_executor().db_reader();
		let (_start, _end, block_event) = db_reader.get_block_info_by_version(0)?;
		Ok(block_event.hash()?)
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
		fs::create_dir_all(
			file_path
				.parent()
				.context("failed to get parent directory for file that should be copied")?,
		)
		.context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should be copied")?;

		// write a file that should not be copied
		let file_path = temp_dir.path().join("celestia").join("test_file2.txt");
		fs::create_dir_all(
			file_path
				.parent()
				.context("failed to get parent directory for file that should not be copied")?,
		)
		.context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should not be copied")?;

		// create the target temp dir
		let dst = TempDir::new()?;

		// copy the file to a new dir, ignoring celestia directory
		copy_dir_recursive_with_ignore(&temp_dir.path(), [PathBuf::from("celestia")], &dst.path())
			.context("failed to copy directory")?;

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

		let path_that_must_be_ignored = source_dir
			.path()
			.join(".movement/celestia/c1860ae680eb2d91927b/.celestia-app/keyring-test");

		fs::create_dir_all(
			path_that_must_be_ignored
				.parent()
				.context("failed to get parent directory for path that must be ignored")?,
		)
		.context("failed to create directory")?;
		// write a file that must not be ignored
		fs::write(path_that_must_be_ignored.clone(), "test")
			.context("failed to write file that must not be ignored")?;
		// set permissions to 000 on the file and then on the parent directory
		fs::set_permissions(path_that_must_be_ignored.clone(), Permissions::from_mode(0o000))
			.context("failed to set permissions on file that must not be ignored")?;
		fs::set_permissions(
			path_that_must_be_ignored
				.parent()
				.context("failed to get parent directory for path that must be ignored")?,
			Permissions::from_mode(0o000),
		)
		.context("failed to set permissions on parent directory that must not be ignored")?;

		copy_dir_recursive_with_ignore(source_dir.path(), ["celestia"], target_dir.path())
			.context("failed to copy directory")?;

		assert!(!target_dir
			.path()
			.join("celestia")
			.join("c1860ae680eb2d91927b")
			.join(".celestia-app")
			.join("keyring-test")
			.exists());

		Ok(())
	}
}
