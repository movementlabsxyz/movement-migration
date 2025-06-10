use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;
use tracing::info;

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
///
/// Note: we use parsing to limit the number of network requests that we make, but the expected values are known a priori.
pub struct ParseRestApi {
	/// The listen url of the rest api known a priori
	known_listen_url: String,
	/// Whether to ping the rest api to ensure it is responding to pings
	ping: bool,
}

impl ParseRestApi {
	pub fn new(listen_url: String, ping: bool) -> Self {
		Self { known_listen_url: listen_url, ping }
	}

	#[cfg(test)]
	pub fn test() -> Self {
		Self { known_listen_url: "http://0.0.0.0:30731".to_string(), ping: false }
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
			if let Some(_captures) =
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
							loop {
								info!("pinging rest api at {}", &self.known_listen_url);
								// check that the endpoint is responding to pings
								// todo: it's actually not true that opt always comes second
								// todo: we should be able to know the port aprior, but this is a hack to test a few other things
								let client = reqwest::Client::new();
								match client.get(&self.known_listen_url).send().await.map_err(|e| {
									FulfillError::Internal(
										format!("failed to ping rest api: {e}").into(),
									)
								}) {
									Ok(response) => {
										if response.status().is_success() {
											break;
										} else {
											info!(
												"rest api is not responding to pings: {response:?}"
											);
											tokio::time::sleep(tokio::time::Duration::from_secs(1))
												.await;
										}
									}
									Err(e) => {
										info!("failed to ping rest api: {e}");
										tokio::time::sleep(tokio::time::Duration::from_secs(1))
											.await;
									}
								}
							}
						}

						return Ok(Some(RestApi { listen_url: self.known_listen_url.clone() }));
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
		let (sender, mut receiver) = tokio::sync::mpsc::channel(2);
		let processor = ParseRestApi::test();

		sender.send(String::from("movement-full-node               | 2025-05-06T08:49:06.205999Z  INFO poem::server: listening addr=socket://0.0.0.0:30731")).await?;

		sender.send(String::from("movement-full-node               | 2025-05-06T08:49:06.205999Z  INFO poem::server: listening addr=socket://0.0.0.0:30731")).await?;

		let result = processor.process_receiver(&mut receiver).await?;
		assert_eq!(result, Some(RestApi { listen_url: "http://0.0.0.0:30731".to_string() }));

		Ok(())
	}
}
