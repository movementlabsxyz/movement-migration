pub const CONTAINER_REPO: &str = "ghcr.io/movementlabsxyz";
pub const CONTAINER_REV: &str = "32234a3636946a8795bc1ee7d290d702414b174a";

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

pub const VALIDATOR: Container =
	Container { repo: CONTAINER_REPO, name: "validator", rev: CONTAINER_REV };

pub const CONTAINERS: &[Container] = &[VALIDATOR];
