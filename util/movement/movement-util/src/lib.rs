pub const CONTAINER_REPO: &str = "ghcr.io/movementlabsxyz";
pub const CONTAINER_REV: &str = "c2372ff";

#[derive(Debug, Clone)]
pub struct Container<'a> {
	repo: &'a str,
	name: &'a str,
	rev: &'a str,
}

impl<'a> Container<'a> {
	pub fn new(repo: &'a str, name: &'a str, rev: &'a str) -> Self {
		Self { repo, name, rev }
	}

	pub fn to_string(&self) -> String {
		format!("{}/{}:{}", self.repo, self.name, self.rev)
	}
}

//   "movement-full-node"
// "movement-celestia-da-light-node"
// "movement-full-node-setup"
// "movement-faucet-service"
// "movement-celestia-bridge"
// "movement-celestia-appd"
// "wait-for-celestia-light-node"

pub const CONTAINERS: &[Container] = &[
	Container { repo: CONTAINER_REPO, name: "movement-full-node", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-celestia-da-light-node", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-full-node-setup", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-faucet-service", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-celestia-bridge", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-celestia-appd", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "wait-for-celestia-light-node", rev: CONTAINER_REV },
];
