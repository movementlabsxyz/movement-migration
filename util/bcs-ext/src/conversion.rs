use bcs;
use serde::{de::DeserializeOwned, Serialize};
/// Errors thrown when trying to use BCS to convert between types.
#[derive(Debug, thiserror::Error)]
pub enum BcsSerializationConversionError {
	#[error("failed to convert via bcs: {0}")]
	Conversion(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("encountered an internal error while converting via bcs: {0}")]
	Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// A trait for converting between types using BCS serialization.
pub trait BcsInto<T> {
	fn bcs_into(self) -> Result<T, BcsSerializationConversionError>;
}

impl<F, T> BcsInto<T> for F
where
	F: Serialize,
	T: DeserializeOwned,
{
	fn bcs_into(self) -> Result<T, BcsSerializationConversionError> {
		bcs::to_bytes(&self)
			.map_err(|e| BcsSerializationConversionError::Conversion(e.into()))
			.and_then(|bytes| {
				bcs::from_bytes::<T>(&bytes)
					.map_err(|e| BcsSerializationConversionError::Conversion(e.into()))
			})
	}
}
