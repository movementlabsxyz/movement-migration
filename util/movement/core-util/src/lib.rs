pub use maptos_opt_executor::{self, *};
pub use movement_client::{self, *};
pub use movement_config;
pub use movement_full_node::{self, *};

pub const CONTAINER_REPO: &str = "ghcr.io/movementlabsxyz";
pub const CONTAINER_REV: &str = "278863e";

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

pub const CONTAINERS: &[Container] = &[
	Container { repo: CONTAINER_REPO, name: "movement-full-node", rev: CONTAINER_REV },
	Container { repo: CONTAINER_REPO, name: "movement-faucet-service", rev: CONTAINER_REV },
];
