use super::MovementNode;
use mtma_types::movement::aptos_types::{
	block_executor::partitioner::{ExecutableBlock, ExecutableTransactions},
	transaction::{signature_verified_transaction::into_signature_verified_block, Version},
};

impl MovementNode {
	/// Iterates over all blocks in the db.
	pub fn iter_blocks(&self, start_version: u64) -> Result<BlockIterator<'_>, anyhow::Error> {
		let latest_version = self.latest_ledger_version()?;
		Ok(BlockIterator { executor: self, version: start_version, latest_version })
	}
}

pub struct BlockIterator<'a> {
	pub(crate) executor: &'a MovementNode,
	pub(crate) version: u64,
	pub(crate) latest_version: u64,
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
