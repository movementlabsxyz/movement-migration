use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

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
pub struct ParseFaucet {
	ping: bool,
}

impl ParseFaucet {
	pub fn new() -> Self {
		Self { ping: true }
	}

	#[cfg(test)]
	pub fn test() -> Self {
		Self { ping: false }
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
					// check that the endpoint is responding to pings
					let client = reqwest::Client::new();
					client
						.get(format!("http://{}:{}", &captures[1], &captures[2]))
						.send()
						.await
						.map_err(|e| {
							FulfillError::Internal(format!("failed to ping faucet: {e}").into())
						})?;
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
		let processor = ParseFaucet::test();

		sender.send(String::from("[movement-faucet] 2025-05-02T07:06:52.984440Z [main] INFO /Users/l-monninger/.cargo/registry/src/index.crates.io-1949cf8c6b557f/poem-1.3.59/src/server.rs {\"addr\":\"socket://0.0.0.0:30732\",\"message\":\"listening\"}")).await?;

		let result = processor.process_receiver(&mut receiver).await?;
		assert_eq!(result, Some(Faucet { listen_url: "http://0.0.0.0:30732".to_string() }));

		Ok(())
	}
}
