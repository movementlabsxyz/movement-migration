use anyhow::Context;
use aptos_config::config::StorageDirPaths;
use aptos_crypto::HashValue;
use aptos_db::AptosDB;
use aptos_executor::db_bootstrapper::generate_waypoint;
use aptos_executor::db_bootstrapper::maybe_bootstrap;
use aptos_executor_types::BlockExecutorTrait;
use aptos_storage_interface::state_store::state_view::db_state_view::DbStateViewAtVersion;
use aptos_storage_interface::DbReaderWriter;
use aptos_types::on_chain_config::{OnChainConfig, OnChainExecutionConfig};
use aptos_types::transaction::Transaction;
use aptos_types::transaction::WriteSetPayload;
use bcs_ext::conversion::BcsInto;
use mtma_node_types::executor::movement_executor::maptos_opt_executor::aptos_types::transaction::Transaction as MovementTransaction;
use mtma_node_types::executor::{
	movement_aptos_executor::{AptosVMBlockExecutor, MovementAptosBlockExecutor},
	MovementAptosNode, MovementNode,
};
use mtma_node_types::{
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
use std::path::Path;
use tracing::info;

/// Converts a [MovementBlock] to a [MovementAptosBlock].
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
pub struct Migrate {
	/// Whether to use the migrated genesis.
	pub use_migrated_genesis: bool,
}

impl Migrationish for Migrate {
	async fn migrate(
		&self,
		movement_node: &MovementNode,
	) -> Result<MovementAptosNode, MigrationError> {
		// open up a new db
		let unique_id = uuid::Uuid::new_v4();
		let timestamp = chrono::Utc::now().timestamp_millis();
		let db_dir = Path::new(".debug").join(format!(
			"migration-db-{}-{}",
			timestamp,
			unique_id.to_string().split('-').next().unwrap()
		));
		let movement_aptos_db = AptosDB::open(
			StorageDirPaths::from_path(db_dir.clone()),
			false,
			Default::default(),
			Default::default(),
			false,
			movement_node.opt_executor().node_config.storage.buffered_state_target_items,
			movement_node
				.opt_executor()
				.node_config
				.storage
				.max_num_nodes_per_lru_cache_shard,
			None,
		)
		.context("failed to open aptos db")
		.map_err(|e| MigrationError::Internal(e.into()))?;

		// form the db reader writer
		let db_rw = DbReaderWriter::new(movement_aptos_db);

		let genesis_txn: Transaction = if self.use_migrated_genesis {
			let movement_genesis_txn = movement_node.genesis_transaction().map_err(|e| {
				MigrationError::Internal(format!("failed to get genesis transaction: {}", e).into())
			})?;

			// extract the write set payload
			let write_set_payload: WriteSetPayload = match movement_genesis_txn {
				MovementTransaction::GenesisTransaction(write_set_payload) => {
					write_set_payload.bcs_into().map_err(|e| {
						MigrationError::Internal(
							format!("failed to extract change set: {}", e).into(),
						)
					})?
				}
				val => {
					return Err(MigrationError::Internal(
						format!("failed to extract change set: {:?}", val).into(),
					))
				}
			};

			Transaction::GenesisTransaction(write_set_payload)
		} else {
			let genesis = aptos_vm_genesis::test_genesis_change_set_and_validators(Some(1));
			Transaction::GenesisTransaction(WriteSetPayload::Direct(genesis.0))
		};

		// generate the waypoint
		let waypoint = generate_waypoint::<AptosVMBlockExecutor>(&db_rw, &genesis_txn)
			.context("failed to generate waypoint")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// maybe bootstrap the aptos db
		let ledger_info_with_sigs =
			maybe_bootstrap::<AptosVMBlockExecutor>(&db_rw, &genesis_txn, waypoint)
				.context("failed to bootstrap")
				.map_err(|e| MigrationError::Internal(e.into()))?
				.context("no ledger info with sigs")
				.map_err(|e| MigrationError::Internal(e.into()))?;

		// form the executor
		let movement_aptos_executor = MovementAptosBlockExecutor::new(db_rw);

		movement_aptos_executor
			.reset()
			.context("failed to reset")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		movement_aptos_executor
			.commit_ledger(ledger_info_with_sigs)
			.context("failed to commit ledger")
			.map_err(|e| MigrationError::Internal(e.into()))?;

		// re-execute the blocks
		for res in movement_node.iter_blocks(1).map_err(|e| {
			MigrationError::Internal(format!("failed to iterate over blocks: {}", e).into())
		})? {
			let (start_version, end_version, block) = res
				.context("failed to get block while iterating over blocks")
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let db_reader = movement_aptos_executor.db.reader.clone();

			// get the latest ledger version
			let latest_ledger_version = db_reader
				.get_latest_ledger_info_version()
				.context("failed to get latest ledger version")
				.map_err(|e| MigrationError::Internal(e.into()))?;

			info!(
				"latest_ledger_version: {}, start_version: {}, end_version: {}",
				latest_ledger_version, start_version, end_version
			);

			// form the state view
			let state_view = db_reader
				.state_view_at_version(Some(latest_ledger_version + 1))
				.map_err(|e| MigrationError::Internal(e.into()))?;

			let block_executor_onchain_config = OnChainExecutionConfig::fetch_config(&state_view)
				.context("onchain execution config not found; it should exist")
				.map_err(|e| MigrationError::Internal(e.into()))?
				.block_executor_onchain_config();

			// convert the movement block to a movement aptos block
			let movement_aptos_block = movement_block_to_movement_aptos_block(block)?;

			// get the parent block id
			let parent_block_id = movement_aptos_executor.committed_block_id();

			movement_aptos_executor
				.execute_and_update_state(
					movement_aptos_block,
					parent_block_id,
					block_executor_onchain_config,
				)
				.context("failed to execute and update state")
				.map_err(|e| MigrationError::Internal(e.into()))?;
		}

		Ok(MovementAptosNode::new(movement_aptos_executor, db_dir))
	}
}
