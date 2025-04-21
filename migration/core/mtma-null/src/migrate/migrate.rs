use aptos_config::config::StorageDirPaths;
use aptos_db::AptosDB;
use aptos_storage_interface::DbReaderWriter;
use migration_executor_types::executor::{
	movement_aptos_executor::MovementAptosBlockExecutor, MovementAptosExecutor, MovementExecutor,
};
use migration_executor_types::migration::{MigrationError, Migrationish};

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copies a directory recursively.
///
/// todo: move this out of the migration module
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
	for entry in WalkDir::new(src) {
		let entry = entry?;
		let rel_path = entry.path().strip_prefix(src).unwrap();
		let dest_path = dst.join(rel_path);

		if entry.file_type().is_dir() {
			fs::create_dir_all(&dest_path)?;
		} else {
			fs::copy(entry.path(), &dest_path)?;
		}
	}
	Ok(())
}

/// Errors thrown during the migration.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
	#[error("failed to migrate: {0}")]
	Migrate(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The migration struct will be use to run a migration from Movement
#[derive(Debug, Clone)]
pub struct Migrate;

impl Migrationish for Migrate {
	async fn migrate(
		&self,
		movement_executor: &MovementExecutor,
	) -> Result<MovementAptosExecutor, MigrationError> {
		// Get the db path from the opt executor.
		let old_db_dir = movement_executor
			.opt_executor()
			.config
			.chain
			.maptos_db_path
			.as_ref()
			.ok_or(MigrationError::Internal("No db path provided.".into()))?;

		// copy all the contents of the db to a timestamp suffixed subdir of .debug
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
		let src = Path::new(old_db_dir);
		let dst = db_dir.join("db");
		copy_dir_recursive(src, &dst).map_err(|e| MigrationError::Internal(e.into()))?;

		// Open the aptos db.
		let aptos_db = AptosDB::open(
			StorageDirPaths::from_path(db_dir.clone()),
			false,
			Default::default(),
			Default::default(),
			false,
			movement_executor.opt_executor().node_config.storage.buffered_state_target_items,
			movement_executor
				.opt_executor()
				.node_config
				.storage
				.max_num_nodes_per_lru_cache_shard,
			None,
		)
		.map_err(|e| MigrationError::Internal(e.into()))?;

		// form the db reader writer
		let db_rw = DbReaderWriter::new(aptos_db);

		// form the executor
		let aptos_executor = MovementAptosBlockExecutor::new(db_rw);

		Ok(MovementAptosExecutor::new(aptos_executor))
	}
}

impl Migrate {
	/// Run the migration.
	///
	/// Note: we will use `run` or a domain-specific term for the core structs in our system,
	/// and `execute` for the CLI structs in our system.
	pub async fn run(&self) -> Result<(), MigrateError> {
		unimplemented!()
	}
}
