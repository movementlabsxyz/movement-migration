use std::fmt::Debug;

/// Errors thrown when attempting to use a runtime.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("encountered internal error while using Movement Aptos Runtime: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("requested Movement Aptos Runtime is unavailable: {0}")]
	Unavailable(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Returns true if the current runtime is multithreaded.
fn is_multithreaded_runtime() -> bool {
	std::panic::catch_unwind(|| {
		tokio::task::block_in_place(|| {});
	})
	.is_ok()
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

/// Tokio test runtime.
#[derive(Debug, Clone)]
pub struct TokioTest;

impl Runtime for TokioTest {
	/// Try to create a new runtime.
	fn try_new() -> Result<Self, RuntimeError> {
		if !is_multithreaded_runtime() {
			return Err(RuntimeError::Unavailable(Box::new(std::io::Error::new(
				std::io::ErrorKind::Other,
				"Tokio test runtime is not multithreaded use #[tokio::test(flavor = \"multi_thread\")] instead",
			))));
		}

		Ok(Self)
	}

	/// Whether to create a global rayon pool.
	fn create_global_rayon_pool() -> bool {
		false
	}
}

/// Native runtime refers to a runtime where the global rayon pool is created within the runner.
#[derive(Debug, Clone)]
pub struct Native;

impl Runtime for Native {
	/// Try to create a new runtime.
	///
	/// There are no restrictions on the surrounding environment here.
	fn try_new() -> Result<Self, RuntimeError> {
		Ok(Self)
	}

	/// Whether to create a global rayon pool.
	fn create_global_rayon_pool() -> bool {
		true
	}
}

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
