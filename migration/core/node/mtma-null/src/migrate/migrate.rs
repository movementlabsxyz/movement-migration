use mtma_node_types::executor::{
	movement_aptos_executor::MovementAptosBlockExecutor, MovementAptosNode, MovementNode,
};
use mtma_node_types::migration::{MigrationError, Migrationish};
use mtma_types::movement_aptos::aptos_config::config::StorageDirPaths;
use mtma_types::movement_aptos::aptos_db::AptosDB;
use mtma_types::movement_aptos::aptos_storage_interface::DbReaderWriter;

use anyhow::Context;
use std::fs;
use std::path::Path;
use tracing::info;
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
		movement_executor: &MovementNode,
	) -> Result<MovementAptosNode, MigrationError> {
		// Get the db path from the opt executor.
		let old_db_dir = movement_executor
			.opt_executor()
			.config
			.chain
			.maptos_db_path
			.as_ref()
			.context("no db path provided.")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// copy all the contents of the db to a timestamp suffixed subdir of .debug
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));

		info!("Copying db to {}", db_dir.display());
		let src = Path::new(old_db_dir);
		copy_dir_recursive(src, &db_dir)
			.context("failed to copy db")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// Open the aptos db.
		info!("Opening aptos db");
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
		.context("failed to open aptos db")
		.map_err(|e| MigrationError::Internal(e.into()))?;

		// form the db reader writer
		let db_rw = DbReaderWriter::new(aptos_db);

		// form the executor
		let aptos_executor = MovementAptosBlockExecutor::new(db_rw);

		Ok(MovementAptosNode::new(aptos_executor, db_dir))
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
