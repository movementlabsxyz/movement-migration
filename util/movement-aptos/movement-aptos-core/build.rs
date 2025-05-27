// Clones https://github.com/movementlabsxyz/aptos-core/commit/e7ad67cfaa13127d41be7eb3b889e62efa7a2bf9
// Runs cargo build -p aptos in it and copies the aptos binary to a known location in target
// Caches to avoid rebuilding unless the provided hash has changed
// Uses hash to ensure cash collisions are properly detected.
// There are a lot of issues with aptos-core builds in nix with crane and similar, so this is a bit of a work around.

use cargo_metadata::MetadataCommand;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const APTOS_CORE_REPO: &str = "https://github.com/movementlabsxyz/aptos-core";
const APTOS_CORE_COMMIT: &str = "fbfde356df4b1316995d03d59542005f1b277f66";
const CACHE_DIR: &str = "aptos_build_cache";

fn main() {
	// Get workspace metadata
	let metadata = MetadataCommand::new().exec().expect("Failed to get cargo metadata");

	// Get the workspace root
	let workspace_root = metadata.workspace_root;
	let workspace_release_dir = workspace_root.join("target/release");

	// ensure the workspace release directory exists
	fs::create_dir_all(&workspace_release_dir)
		.expect("Failed to create workspace release directory");

	// Get the out directory
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let cache_path = out_dir.join(CACHE_DIR);
	let commit_cache = cache_path.join(format!(
		"{}_{}",
		APTOS_CORE_COMMIT,
		env::var("TARGET").unwrap_or_default()
	));
	let aptos_bin_path = commit_cache.join("target/release/aptos");

	if fs::metadata(&aptos_bin_path).is_ok() {
		println!("cargo:warning=Using cached aptos binary");
	} else {
		println!("cargo:warning=No cache found, building from scratch");

		// ensure the parent directory for the commit cache exists
		let parent_dir = commit_cache.parent().unwrap();
		fs::create_dir_all(parent_dir).expect("Failed to create parent directory for commit cache");

		// clone if not exists
		if !fs::metadata(&commit_cache).is_ok() {
			println!("cargo:warning=Cloning repository...");
			Command::new("git")
				.args(&["clone", APTOS_CORE_REPO, &commit_cache.to_string_lossy()])
				.status()
				.expect("Failed to clone repository");
		}

		// check out the desired commit
		println!("cargo:warning=Checking out commit {}", APTOS_CORE_COMMIT);
		Command::new("git")
			.args(&["checkout", APTOS_CORE_COMMIT])
			.current_dir(&commit_cache)
			.status()
			.expect("Failed to checkout commit");

		// build the aptos binary
		println!("cargo:warning=Building aptos binary...");
		Command::new("cargo")
			.current_dir(&commit_cache)
			.args(&["build", "-p", "aptos", "--release"])
			.status()
			.expect("Failed to build aptos");
	}

	// copy aptos binary to the workspace release directory
	fs::copy(&aptos_bin_path, &workspace_release_dir.join("aptos"))
		.expect("Failed to copy aptos binary to workspace release directory");
}
