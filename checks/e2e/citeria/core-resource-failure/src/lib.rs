use migration_e2e_test_types::criterion::{
	movement_aptos_e2e_client::types::LocalAccount as MovementAptosLocalAccount,
	movement_e2e_client::types::LocalAccount as MovementLocalAccount, Criterion, CriterionError,
	Criterionish, MovementAptosE2eClient, MovementE2eClient,
};

pub mod bridge_scripts {
	// Define a struct to hold script information
	#[derive(Debug)]
	pub struct BridgeScript {
		pub code: &'static [u8],
		pub name: &'static str,
	}

	macro_rules! bridge_script {
		($name:literal) => {
			BridgeScript {
				code: include_bytes!(concat!("include/bridge-scripts/", $name)),
				name: $name,
			}
		};
	}

	// Create a const array of script information
	pub const BRIDGE_SCRIPTS: &[BridgeScript] = &[
		bridge_script!("enable_bridge_feature.move"),
		bridge_script!("store_mint_burn_caps.move"),
		bridge_script!("update_bridge_fee.move"),
		bridge_script!("update_bridge_relayer.move"),
	];
}

pub struct CoreResourceScriptForbidden {
	/// The movement local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	movement_local_account: MovementLocalAccount,
	/// The aptos local account
	///
	/// TODO: we may want to switch this to the secure signing API if the account is sensitive.
	aptos_local_account: MovementAptosLocalAccount,
}

impl CoreResourceScriptForbidden {
	pub fn new(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Self {
		Self { movement_local_account, aptos_local_account }
	}

	pub fn criterion(
		movement_local_account: MovementLocalAccount,
		aptos_local_account: MovementAptosLocalAccount,
	) -> Criterion<Self> {
		Criterion::new(Self { movement_local_account, aptos_local_account })
	}

	pub fn movement_local_account(&self) -> &MovementLocalAccount {
		&self.movement_local_account
	}

	pub fn movement_local_account_mut(&mut self) -> &mut MovementLocalAccount {
		&mut self.movement_local_account
	}

	pub fn aptos_local_account(&self) -> &MovementAptosLocalAccount {
		&self.aptos_local_account
	}

	pub fn aptos_local_account_mut(&mut self) -> &mut MovementAptosLocalAccount {
		&mut self.aptos_local_account
	}
}

impl Criterionish for CoreResourceScriptForbidden {
	async fn satisfies(
		&mut self,
		movement_client: &MovementE2eClient,
		movement_aptos_client: &MovementAptosE2eClient,
	) -> Result<(), CriterionError> {
		let mut violations = vec![];

		for bridge_script in bridge_scripts::BRIDGE_SCRIPTS {
			let move_result = movement_client.simulate_script(
				self.movement_local_account_mut(),
				bridge_script.code,
				vec![],
			);
			if move_result.is_ok() {
				violations.push(format!("movement_client allowed {}", bridge_script.name));
			}

			let maptos_result = movement_aptos_client.simulate_script(
				self.aptos_local_account_mut(),
				bridge_script.code,
				vec![],
			);

			if maptos_result.is_ok() {
				violations.push(format!("movement_aptos_client allowed {}", bridge_script.name));
			}
		}

		if !violations.is_empty() {
			return Err(CriterionError::Unsatisfied(
				format!(
					"Core resource scripts should not be executable by unauthorized signers:\n{}",
					violations.join("\n")
				)
				.into(),
			));
		}

		Ok(())
	}
}
