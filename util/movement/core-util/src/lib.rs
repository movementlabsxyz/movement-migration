pub use maptos_opt_executor::*;
pub use movement_client::{self, *};
pub use movement_util::{self, *};
use vendor_util::VendorPlan;

pub const CONTAINER_REPO: &str = "ghcr.io/movementlabsxyz";
pub const CONTAINER_REV: &str = "c2372ff";

#[derive(Debug, Clone)]
pub struct Container {
	repo: String,
	name: String,
	rev: String,
}

impl Container {
	pub fn new(repo: String, name: String, rev: String) -> Self {
		Self { repo, name, rev }
	}

	pub fn to_string(&self) -> String {
		format!("{}/{}:{}", self.repo, self.name, self.rev)
	}

	pub fn container_from_vendor_plan(plan: &VendorPlan, name: impl Into<String>) -> Self {
		Self { repo: CONTAINER_REPO.to_string(), name: name.into(), rev: plan.git_rev.clone() }
	}

	pub fn containers_from_vendor_plan(plan: &VendorPlan) -> Vec<Self> {
		vec![
			Self::container_from_vendor_plan(plan, "movement-full-node"),
			Self::container_from_vendor_plan(plan, "movement-celestia-da-light-node"),
			Self::container_from_vendor_plan(plan, "movement-full-node-setup"),
			Self::container_from_vendor_plan(plan, "movement-faucet-service"),
			Self::container_from_vendor_plan(plan, "movement-celestia-bridge"),
			Self::container_from_vendor_plan(plan, "movement-celestia-appd"),
			Self::container_from_vendor_plan(plan, "wait-for-celestia-light-node"),
		]
	}
}
