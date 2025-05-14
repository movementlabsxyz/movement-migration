pub mod delegated;
pub mod native;
pub mod tokio_test;

pub use delegated::*;
pub use native::*;
pub use tokio_test::*;

use std::fmt::Debug;

/// Errors thrown when attempting to use a runtime.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("encountered internal error while using Movement Aptos Runtime: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("requested Movement Aptos Runtime is unavailable: {0}")]
	Unavailable(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Trait for a runtime that can be used to run [MovementAptos].
///
/// A runtime knows statically whether it is multithreaded or not.
pub trait Runtime: Sized + Clone + Debug + Send + Sync + 'static {
	/// Try to create a new runtime.
	fn try_new() -> Result<Self, RuntimeError>;

	/// Returns whether to create a global rayon pool.
	fn create_global_rayon_pool() -> bool;
}

/// Returns true if the current runtime is multithreaded.
fn is_multithreaded_runtime() -> bool {
	std::panic::catch_unwind(|| {
		tokio::task::block_in_place(|| {});
	})
	.is_ok()
}

/// Returns true if the current runtime is a tokio runtime.
fn is_in_tokio_runtime() -> bool {
	tokio::runtime::Handle::try_current().is_ok()
}
