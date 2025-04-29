use bcs;
use serde::Serialize;

/// Errors thrown when trying to compare BCS types.
#[derive(Debug, thiserror::Error)]
pub enum BcsSerializationComparisonError {
	#[error("failed to compare via bcs: {0}")]
	Comparison(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("encountered an internal error while comparing via bcs: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A trait for comparing types using BCS serialization.
pub trait BcsEq<Rhs = Self> {
	fn bcs_eq(&self, other: &Rhs) -> Result<bool, BcsSerializationComparisonError>;
}

impl<T, U> BcsEq<U> for T
where
	T: Serialize,
	U: Serialize,
{
	fn bcs_eq(&self, other: &U) -> Result<bool, BcsSerializationComparisonError> {
		let self_bytes = bcs::to_bytes(self)
			.map_err(|e| BcsSerializationComparisonError::Comparison(e.into()))?;
		let other_bytes = bcs::to_bytes(other)
			.map_err(|e| BcsSerializationComparisonError::Comparison(e.into()))?;
		Ok(self_bytes == other_bytes)
	}
}
