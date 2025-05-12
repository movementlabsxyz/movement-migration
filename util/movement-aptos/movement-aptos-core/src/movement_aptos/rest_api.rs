use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub struct RestApi {
	/// The Rest Api url.
	pub rest_api_url: String,
}

pub struct WaitForRestApi {
	/// The expected [RestApi]
	pub rest_api: RestApi,
}

impl WaitForRestApi {
	pub fn new(rest_api: RestApi) -> Self {
		Self { rest_api }
	}
}

impl CustomProcessor<RestApi> for WaitForRestApi {
	async fn process_receiver(
		&self,
		receiver: &mut Receiver<String>,
	) -> Result<Option<RestApi>, FulfillError> {
		loop {
			// use the receiver to pace the polling
			receiver.recv().await;

			// keep curling the rest api until the faucet is ready
			let response = reqwest::get(self.rest_api.rest_api_url.clone())
				.await
				.map_err(|e| FulfillError::Internal(e.into()))?;
			if response.status().is_success() {
				return Ok(Some(self.rest_api.clone()));
			}
		}
	}
}
