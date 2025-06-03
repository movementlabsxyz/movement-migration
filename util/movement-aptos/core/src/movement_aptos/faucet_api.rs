#[derive(Debug, Clone)]
pub struct FaucetApi {
	/// The Rest Api url.
	pub faucet_api_url: String,
}

impl FaucetApi {
	pub fn new(faucet_api_url: String) -> Self {
		Self { faucet_api_url }
	}

	/// Borrow the rest api listen url.
	pub fn listen_url(&self) -> &str {
		&self.faucet_api_url
	}
}
