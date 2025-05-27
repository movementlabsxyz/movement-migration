use super::{is_in_tokio_runtime, is_multithreaded_runtime, Runtime, RuntimeError};

/// Tokio test runtime.
#[derive(Debug, Clone)]
pub struct TokioTest;

impl Runtime for TokioTest {
	/// Try to create a new runtime.
	fn try_new() -> Result<Self, RuntimeError> {
		if !is_in_tokio_runtime() || !is_multithreaded_runtime() {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test(flavor = "multi_thread")]
	async fn test_tokio_test_should_succeed_in_multi_thread() -> Result<(), anyhow::Error> {
		TokioTest::try_new()?;
		Ok(())
	}

	#[tokio::test]
	async fn test_tokio_test_should_fail_in_single_thread() -> Result<(), anyhow::Error> {
		assert!(TokioTest::try_new().is_err());
		Ok(())
	}

	#[test]
	fn test_tokio_test_should_fail_outside_of_tokio_test() -> Result<(), anyhow::Error> {
		assert!(TokioTest::try_new().is_err());
		Ok(())
	}
}
