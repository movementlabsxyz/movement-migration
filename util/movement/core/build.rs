use include_vendor::{Buildtime, BuildtimeError, Noop};
use movement_core_util::CONTAINERS;
use vendor_util::VendorPlan;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let mut readier: ready_docker::Buildtime<ready_docker::Noop, ready_docker::Noop> =
		ready_docker::Buildtime::new();
	for container in CONTAINERS {
		readier.add_image(container.to_string());
	}
	readier.build().await.map_err(|e| BuildtimeError::Internal(e.into()))?;

	// get the source information from movement-client
	let mut plan = VendorPlan::try_from_cargo_dep("movement-client")?;
	// once we have the source information, rename the vendor to movement
	plan.vendor_name = "movement".to_string();

	let vendor = plan.execute()?;
	let builder: Buildtime<Noop, Noop> = Buildtime::try_new(vendor)?;
	builder.build()?;

	Ok(())
}
