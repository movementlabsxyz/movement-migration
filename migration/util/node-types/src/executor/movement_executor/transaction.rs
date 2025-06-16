use super::MovementNode;
use mtma_types::movement::aptos_types::transaction::Transaction;
use tracing::info;

impl MovementNode {
	/// Iterates over all transactions in the db.
	pub fn iter_transactions(
		&self,
		start_version: u64,
	) -> Result<TransactionIterator<'_>, anyhow::Error> {
		Ok(TransactionIterator::new(self, start_version, self.latest_ledger_version()?))
	}
}

pub struct TransactionIterator<'a> {
	pub(crate) executor: &'a MovementNode,
	pub(crate) version: u64,
	pub(crate) latest_version: u64,
	pub(crate) current_block_iter: Option<Box<dyn Iterator<Item = Transaction> + 'a>>,
}

impl<'a> TransactionIterator<'a> {
	pub(crate) fn new(executor: &'a MovementNode, version: u64, latest_version: u64) -> Self {
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
