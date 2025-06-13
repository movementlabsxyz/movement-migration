use std::path::PathBuf;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct LiveMigrator {
	pub rest_api_url: String,
	pub dir: PathBuf,
}

impl LiveMigrator {
	pub fn new(rest_api_url: String, dir: PathBuf) -> Self {
		Self { rest_api_url, dir }
	}
}

impl LiveMigrator {
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		// checks for liveness by polling the rest api url
		// once it becomes read the firs time, it should not later return an error
		let mut ready = false;
		loop {
			match reqwest::get(self.rest_api_url.clone()).await {
				Ok(response) => {
					if response.status().is_success() {
						ready = true;
						continue;
					} else if response.status().is_server_error() && ready {
						return Err(anyhow::anyhow!("live migrator failed health check"));
					} else {
						warn!("live migrator not yet ready: response: {:?}", response);
					}
				}
				Err(e) => {
					warn!("failed to poll live migrator: {}", e);
					if ready {
						return Err(anyhow::anyhow!("live migrator failed health check"));
					}
				}
			}
		}
	}

	pub async fn wait_for_live_rest_api_url(&self) -> Result<String, anyhow::Error> {
		// us request to poll the rest API url until is ready
		loop {
			match reqwest::get(self.rest_api_url.clone()).await {
				Ok(response) => {
					if response.status().is_success() {
						return Ok(self.rest_api_url.clone());
					}
					warn!("rest api url is not ready yet: response: {:?}", response);
				}
				Err(e) => {
					warn!("failed to poll rest api url: {}", e);
				}
			}
		}
	}
}
