use super::{is_in_tokio_runtime, Runtime, RuntimeError};

/// Native runtime refers to a runtime where the global rayon pool is created within the runner.
#[derive(Debug, Clone)]
pub struct Native;

impl Runtime for Native {
	/// Try to create a new runtime.
	///
	/// There are no restrictions on the surrounding environment here.
	fn try_new() -> Result<Self, RuntimeError> {
		if is_in_tokio_runtime() {
			return Err(RuntimeError::Unavailable(Box::new(std::io::Error::new(
				std::io::ErrorKind::Other,
				"Native runtime is not available in a tokio runtime",
			))));
		}

		Ok(Self)
	}

	/// Whether to create a global rayon pool.
	fn create_global_rayon_pool() -> bool {
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_native_runtime_should_fail_in_tokio_runtime() -> Result<(), anyhow::Error> {
		assert!(Native::try_new().is_err());
		Ok(())
	}

	#[test]
	fn test_native_runtime_should_succeed_outside_of_tokio_runtime() -> Result<(), anyhow::Error> {
		assert!(Native::try_new().is_ok());
		Ok(())
	}
}
