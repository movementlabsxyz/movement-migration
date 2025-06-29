use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Faucet {
	listen_url: String,
}

impl Faucet {
	pub fn listen_url(&self) -> &str {
		&self.listen_url
	}
}

/// A struct used to parse the output of Movement for Faucet data.
///
/// Note: we use parsing to limit the number of network requests that we make, but the expected values are known a priori.
pub struct ParseFaucet {
	/// The listen url of the faucet known a priori
	known_listen_url: String,
	/// Whether to ping the faucet to ensure it is responding to pings
	ping: bool,
}

impl ParseFaucet {
	pub fn new(known_listen_url: String, ping: bool) -> Self {
		Self { known_listen_url, ping }
	}

	#[cfg(test)]
	pub fn test(known_listen_url: String) -> Self {
		Self { known_listen_url, ping: false }
	}
}

impl CustomProcessor<Faucet> for ParseFaucet {
	async fn process_receiver(
		&self,
		receiver: &mut Receiver<String>,
	) -> Result<Option<Faucet>, FulfillError> {
		while let Some(line) = receiver.recv().await {
			let stripped = strip_ansi_escapes::strip(&line);
			let trimmed = std::str::from_utf8(&stripped)
				.map_err(|e| FulfillError::Internal(format!("invalid UTF-8: {e}").into()))?
				.trim();

			// match line of the following form to extract the faucet url: [movement-faucet        ] 2025-05-02T07:06:52.984440Z [main] INFO /Users/l-monninger/.cargo/registry/src/index.crates.io-1949cf8c6b557f/poem-1.3.59/src/server.rs {"addr":"socket://0.0.0.0:30732","message":"listening"}
			if let Some(captures) =
				regex::Regex::new(r#"movement-faucet.*socket://([^:]+):(\d+).*listening"#)
					.map_err(|e| FulfillError::Internal(format!("invalid regex: {e}").into()))?
					.captures(&trimmed)
			{
				if self.ping {
					loop {
						info!("pinging faucet at {}", &self.known_listen_url);
						// check that the endpoint is responding to pings
						let client = reqwest::Client::new();
						match client.get(&self.known_listen_url).send().await.map_err(|e| {
							FulfillError::Internal(format!("failed to ping faucet: {e}").into())
						}) {
							Ok(response) => {
								if response.status().is_success() {
									info!("faucet is responding to pings");
									break;
								} else {
									info!("faucet is not responding to pings");
								}
							}
							Err(e) => {
								warn!("failed to ping faucet: {e}");
							}
						}
						tokio::time::sleep(Duration::from_secs(1)).await;
					}
				}

				return Ok(Some(Faucet {
					listen_url: format!("http://{}:{}", &captures[1], &captures[2]),
				}));
			}
		}

		Ok(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_faucet() -> Result<(), anyhow::Error> {
		let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
		let processor = ParseFaucet::test("http://0.0.0.0:30732".to_string());

		sender.send(String::from("[movement-faucet] 2025-05-02T07:06:52.984440Z [main] INFO /Users/l-monninger/.cargo/registry/src/index.crates.io-1949cf8c6b557f/poem-1.3.59/src/server.rs {\"addr\":\"socket://0.0.0.0:30732\",\"message\":\"listening\"}")).await?;

		let result = processor.process_receiver(&mut receiver).await?;
		assert_eq!(result, Some(Faucet { listen_url: "http://0.0.0.0:30732".to_string() }));

		Ok(())
	}
}
