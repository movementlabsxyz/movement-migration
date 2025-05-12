use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub struct RestApi {
	/// The Rest Api url.
	pub rest_api_url: String,
}

impl RestApi {
	pub fn new(rest_api_url: String) -> Self {
		Self { rest_api_url }
	}

	/// Borrow the rest api listen url.
	pub fn listen_url(&self) -> &str {
		&self.rest_api_url
	}
}
