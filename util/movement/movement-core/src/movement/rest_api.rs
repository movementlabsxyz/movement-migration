use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RestApi {
	listen_url: String,
}

impl RestApi {
	pub fn listen_url(&self) -> &str {
		&self.listen_url
	}
}

/// Tracks the progress of the rest api logging.
///
/// We should see a fin API registered then an opt API registered.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RestApiProgress {
	WaitingForFin,
	WaitingForOpt,
}

impl RestApiProgress {
	pub fn start() -> Self {
		Self::WaitingForFin
	}
}

/// A struct used to parse the output of Movement for RestApi data.
pub struct ParseRestApi {
	ping: bool,
}

impl ParseRestApi {
	pub fn new() -> Self {
		Self { ping: true }
	}

	#[cfg(test)]
	pub fn test() -> Self {
		Self { ping: false }
	}
}

impl CustomProcessor<RestApi> for ParseRestApi {
	async fn process_receiver(
		&self,
		receiver: &mut Receiver<String>,
	) -> Result<Option<RestApi>, FulfillError> {
		let mut progress = RestApiProgress::start();

		while let Some(line) = receiver.recv().await {
			let stripped = strip_ansi_escapes::strip(&line);
			let trimmed = std::str::from_utf8(&stripped)
				.map_err(|e| FulfillError::Internal(format!("invalid UTF-8: {e}").into()))?
				.trim();

			// match line of the following form to extract the rest api listen url: movement-full-node | 2025-05-06T08:49:06.205999Z  INFO poem::server: listening addr=socket://0.0.0.0:30731
			if let Some(captures) =
				regex::Regex::new(r#"movement-full-node.*socket://([^:]+):(\d+)"#)
					.map_err(|e| FulfillError::Internal(format!("invalid regex: {e}").into()))?
					.captures(&trimmed)
			{
				match progress {
					RestApiProgress::WaitingForFin => {
						progress = RestApiProgress::WaitingForOpt;
					}
					RestApiProgress::WaitingForOpt => {
						if self.ping {
							// check that the endpoint is responding to pings
							let client = reqwest::Client::new();
							client
								.get(format!("http://{}:{}", &captures[1], &captures[2]))
								.send()
								.await
								.map_err(|e| {
									FulfillError::Internal(
										format!("failed to ping rest api: {e}").into(),
									)
								})?;
						}

						return Ok(Some(RestApi {
							listen_url: format!("http://{}:{}", &captures[1], &captures[2]),
						}));
					}
				}
			}
		}

		Ok(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_rest_api() -> Result<(), anyhow::Error> {
		let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
		let processor = ParseRestApi::test();

		sender.send(String::from("movement-full-node               | 2025-05-06T08:49:06.205999Z  INFO poem::server: listening addr=socket://0.0.0.0:30731")).await?;

		let result = processor.process_receiver(&mut receiver).await?;
		assert_eq!(result, Some(RestApi { listen_url: "http://0.0.0.0:30731".to_string() }));

		Ok(())
	}
}
