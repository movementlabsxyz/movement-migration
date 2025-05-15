use super::{Runtime, RuntimeError};

/// Delegated runtime refers to a runtime where the global rayon pool is created outside of the runner.
///
/// This is useful when we may be calling from other tasks, i.e., not at the main thread.
#[derive(Debug, Clone)]
pub struct Delegated;

impl Runtime for Delegated {
	/// Try to create a new runtime.
	///
	/// There are no restrictions on the surrounding environment here.
	fn try_new() -> Result<Self, RuntimeError> {
		Ok(Self)
	}

	/// Whether to create a global rayon pool.
	fn create_global_rayon_pool() -> bool {
		false
	}
}
