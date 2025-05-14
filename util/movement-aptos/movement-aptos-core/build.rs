// make sure that `movement-aptos` is built from the workspace
use std::process::Command;

fn main() {
	// check that `movement-aptos` exists
	let binary_name =
		if cfg!(target_os = "windows") { "movement-aptos.exe" } else { "movement-aptos" };

	let output = Command::new(binary_name).output().expect("Failed to run 'movement-aptos");
	if !output.status.success() {
		panic!("'movement-aptos' failed to run with status: {}", output.status);
	}
}
