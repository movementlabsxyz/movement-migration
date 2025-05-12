use crate::migrate::Config;

/// Errors thrown during the framework upgrade.
#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
	#[error("failed to upgrade: {0}")]
	Upgrade(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The Migrate struct will be use to run a framework migration.
#[derive(Debug, Clone)]
pub struct Migrate {
	pub config: Config,
}

impl Migrate {
	/// Run the framework migration.
	///
	/// Note: we will use `run` or a domain-specific term for the core structs in our system,
	/// and `execute` for the CLI structs in our system.
	pub async fn run(&self) -> Result<(), anyhow::Error> {
		// Handle the migration directly based on from/to values
		match (self.config.from.as_str(), self.config.to.as_str()) {
			("biarritz-rc1", "pre-l1-merge") | ("pre-l1-merge", "biarritz-rc1") => {
				// Todo: migration logic
				Ok(())
			}
			_ => Err(anyhow::anyhow!(
				"Unsupported migration path: {} -> {}",
				self.config.from,
				self.config.to
			)),
		}
	}
}
