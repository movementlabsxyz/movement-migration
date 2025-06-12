use anyhow::Context;
use clap::Parser;
use mtma_box_environment::Config as BoxEnvironmentConfig;
use mtma_environment_types::Environmentish;
use mtma_migrator_types::migration::Migrationish;
use mtma_node_null_core::Config as NullConfig;
use mtma_provisioner_environment::Config as ProvisionerEnvironmentConfig;
use mtma_testing_environment::Config as TestingEnvironmentConfig;
use serde::{Deserialize, Serialize};
use slect::Slect;

/// Select migration over the node.
#[derive(Parser, Slect, Deserialize, Serialize, Debug, Clone)]
#[clap(help_expected = true)]
pub struct Select {
	/// Extra args to pass to slect API
	#[slect(environment_testing = TestingEnvironmentConfig, environment_box = BoxEnvironmentConfig, environment_provisioner = ProvisionerEnvironmentConfig, null = NullConfig)]
	extra_args: Vec<String>,
}

impl select::Select {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let (
			maybe_environment_testing,
			maybe_environment_box,
			maybe_environment_provisioner,
			maybe_null,
		) = self.select().map_err(|e| anyhow::anyhow!("{}", e))?;

		// if more than one environment is provided, we need to error
		let environment_count = maybe_environment_testing.is_some() as u8
			+ maybe_environment_box.is_some() as u8
			+ maybe_environment_provisioner.is_some() as u8;
		if environment_count > 1 {
			return Err(anyhow::anyhow!(
				"only one environment should be provided, but got {}",
				environment_count
			));
		}

		let environment_config = maybe_environment_testing.context(
			"--environment-testing is the only supported environment; it must be provided",
		)?;

		let movement_migrator = environment_config.build()?.build_movement_migrator().await?;

		if let Some(null) = maybe_null {
			let null_migration = null.build()?;

			null_migration.migrate(&movement_migrator).await?;
		}

		Ok(())
	}
}
