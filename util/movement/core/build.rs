use include_vendor::{Buildtime, BuildtimeError, Noop};
use movement_core_util::Container;
use vendor_util::VendorPlan;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// get the source information from movement-client
	let mut plan = VendorPlan::try_from_cargo_dep("movement-client")?;
	// once we have the source information, rename the vendor to movement
	plan.vendor_name = "movement".to_string();

	// skip if we intend to build within the Docker container.
	if std::env::var("BUILD").unwrap_or_default() != "docker" {
		// ready the docker images
		let mut readier: ready_docker::Buildtime<ready_docker::Noop, ready_docker::Noop> =
			ready_docker::Buildtime::new();
		for container in Container::containers_from_vendor_plan(&plan) {
			readier.add_image(container.to_string());
		}
		readier.build().await.map_err(|e| BuildtimeError::Internal(e.into()))?;
	}

	let vendor = plan.execute()?;
	let builder: Buildtime<Noop, Noop> = Buildtime::try_new(vendor)?;
	builder.build()?;

	Ok(())
}
