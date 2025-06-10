use include_vendor::{Buildtime, BuildtimeError, Noop};
use movement_core_util::CONTAINERS;

#[tokio::main]
async fn main() -> Result<(), BuildtimeError> {
	let mut readier: ready_docker::Buildtime<ready_docker::Noop, ready_docker::Noop> =
		ready_docker::Buildtime::new();
	for container in CONTAINERS {
		readier.add_image(container.to_string());
	}

	// do not ready docker if build is BUILD=docker
	if std::env::var("BUILD").unwrap_or_default() != "docker" {
		readier.build().await.map_err(|e| BuildtimeError::Internal(e.into()))?;
	}

	let builder: Buildtime<Noop, Noop> = Buildtime::try_new("movement".to_string())?;
	builder.build()?;

	Ok(())
}
