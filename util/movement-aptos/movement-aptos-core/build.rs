use movement_aptos_util::CONTAINERS;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let mut readier: ready_docker::Buildtime<ready_docker::Noop, ready_docker::Noop> =
		ready_docker::Buildtime::new();
	for container in CONTAINERS {
		readier.add_image(container.to_string());
	}
	if !readier.build().await.is_err() {
		panic!("Expected error for arm64 pull: this means that the image is now available and you should update this implementation");
	}

	Ok(())
}
