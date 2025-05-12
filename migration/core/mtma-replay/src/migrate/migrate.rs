use aptos_config::config::StorageDirPaths;
use aptos_crypto::HashValue;
use aptos_db::AptosDB;
use aptos_executor::db_bootstrapper::generate_waypoint;
use aptos_executor::db_bootstrapper::maybe_bootstrap;
use aptos_executor_types::BlockExecutorTrait;
use aptos_storage_interface::DbReaderWriter;
use aptos_types::on_chain_config::{OnChainConfig, OnChainExecutionConfig};
use aptos_types::transaction::Transaction;
use migration_executor_types::executor::{
	movement_aptos_executor::{AptosVMBlockExecutor, MovementAptosBlockExecutor},
	MovementAptosExecutor, MovementExecutor,
};
use migration_executor_types::{
	executor::{
		movement_aptos_executor::aptos_types::{
			block_executor::partitioner::{
				ExecutableBlock as MovementAptosBlock, ExecutableTransactions,
			},
			transaction::signature_verified_transaction::SignatureVerifiedTransaction,
		},
		movement_executor::maptos_opt_executor::aptos_types::block_executor::partitioner::ExecutableBlock as MovementBlock,
	},
	migration::{MigrationError, Migrationish},
};

use anyhow::Context;
use aptos_storage_interface::state_store::state_view::db_state_view::DbStateViewAtVersion;
use bcs_ext::conversion::BcsInto;
use std::path::Path;

pub fn movement_block_to_movement_aptos_block(
	block: MovementBlock,
) -> Result<MovementAptosBlock, MigrationError> {
	let hash_value = HashValue::from_slice(&block.block_id.to_vec())
		.map_err(|e| MigrationError::Internal(e.into()))?;

	let transactions = block.transactions.into_txns();

	// serialize each transaction and then deserialize it into the [SignatureVerifiedTransaction]
	let transactions: Result<Vec<SignatureVerifiedTransaction>, MigrationError> = transactions
		.into_iter()
		.map(|txn| {
			let serialized = bcs::to_bytes(&txn).map_err(|e| MigrationError::Internal(e.into()))?;
			let txn: SignatureVerifiedTransaction =
				bcs::from_bytes(&serialized).map_err(|e| MigrationError::Internal(e.into()))?;
			Ok(txn)
		})
		.collect();

	let executable_transactions = ExecutableTransactions::Unsharded(transactions?);

	Ok(MovementAptosBlock::new(hash_value, executable_transactions))
}

/// Errors thrown during the migration.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
	#[error("failed to migrate: {0}")]
	Migrate(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The migration struct will be use to run a migration from Movement to Movement Aptos by replaying all blocks from the Movement Aptos db.
#[derive(Debug, Clone)]
pub struct Migrate;

impl Migrationish for Migrate {
	async fn migrate(
		&self,
		movement_executor: &MovementExecutor,
	) -> Result<MovementAptosExecutor, MigrationError> {
		// open up a new db
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
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

		// add the genesis transaction from the movement db to the aptos db
		let movement_genesis_txn = movement_executor.genesis_transaction().map_err(|e| {
			MigrationError::Internal(format!("failed to get genesis transaction: {}", e).into())
		})?;
		let aptos_genesis_transaction: Transaction =
			movement_genesis_txn.bcs_into().map_err(|e| {
				MigrationError::Internal(
					format!("failed to convert genesis transaction: {}", e).into(),
				)
			})?;

		// generate the waypoint
		let waypoint =
			generate_waypoint::<AptosVMBlockExecutor>(&db_rw, &aptos_genesis_transaction)
				.context("failed to generate waypoint")
				.map_err(|e| MigrationError::Internal(e.into()))?;

		// maybe bootstrap the aptos db
		let ledger_info_with_sigs =
			maybe_bootstrap::<AptosVMBlockExecutor>(&db_rw, &aptos_genesis_transaction, waypoint)
				.context("failed to bootstrap")
				.map_err(|e| MigrationError::Internal(e.into()))?
				.context("no ledger info with sigs")
				.map_err(|e| MigrationError::Internal(e.into()))?;
		println!("ledger_info_with_sigs: {:?}", ledger_info_with_sigs);

		// form the executor
		let aptos_executor = MovementAptosBlockExecutor::new(db_rw.clone());

		aptos_executor
			.reset()
			.context("failed to reset")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		aptos_executor
			.commit_ledger(ledger_info_with_sigs)
			.context("failed to commit ledger")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// re-execute the blocks
		for res in movement_executor.iter_blocks(1).map_err(|e| {
			MigrationError::Internal(format!("failed to iterate over blocks: {}", e).into())
		})? {
			let (start_version, _end_version, block) = res
				.context("failed to get block while iterating over blocks")
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let db_reader = db_rw.reader.clone();

			// get the latest ledger version
			let latest_ledger_version = db_reader
				.get_latest_ledger_info_version()
				.context("failed to get latest ledger version")
				.map_err(|e| MigrationError::Internal(e.into()))?;

			// form the state view
			let state_view = db_reader
				.state_view_at_version(Some(latest_ledger_version))
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let block_executor_onchain_config = OnChainExecutionConfig::fetch_config(&state_view)
				.unwrap_or_else(OnChainExecutionConfig::default_if_missing)
				.block_executor_onchain_config();

			let movement_aptos_block = movement_block_to_movement_aptos_block(block)?;

			aptos_executor
				.execute_and_update_state(
					movement_aptos_block,
					HashValue::zero(),
					block_executor_onchain_config,
				)
				.context("failed to execute and update state")
				.map_err(|e| MigrationError::Internal(e.into()))?;
		}

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
