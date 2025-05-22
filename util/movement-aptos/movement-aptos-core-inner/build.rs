// Clones https://github.com/movementlabsxyz/aptos-core/commit/e7ad67cfaa13127d41be7eb3b889e62efa7a2bf9
// Runs cargo build -p aptos in it and copies the aptos binary to a known location in target
// Caches to avoid rebuilding unless the provided hash has changed
// Uses hash to ensure cash collisions are properly detected.
// There are a lot of issues with aptos-core builds in nix with crane and similar, so this is a bit of a work around.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const APTOS_CORE_REPO: &str = "https://github.com/movementlabsxyz/aptos-core";
const APTOS_CORE_COMMIT: &str = "fbfde356df4b1316995d03d59542005f1b277f66";
const CACHE_FILE: &str = "aptos_build_cache";

fn main() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let cache_path = out_dir.join(CACHE_FILE);
	let aptos_bin_path = out_dir.join("aptos");

	// Create a hash of the commit we want to build
	let hash = format!("{}_{}", APTOS_CORE_COMMIT, env::var("TARGET").unwrap_or_default());

	// Check if we have a cached build
	if let Ok(cached_hash) = fs::read_to_string(&cache_path) {
		if cached_hash.trim() == hash {
			println!("cargo:warning=Using cached aptos binary");
			return;
		}
	}

	// Create a temporary directory for the build
	let temp_dir = out_dir.join("aptos-core-build");
	if temp_dir.exists() {
		fs::remove_dir_all(&temp_dir).expect("Failed to remove existing temp directory");
	}
	fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");

	// Clone the repository
	println!("cargo:warning=Cloning aptos-core repository...");
	let status = Command::new("git")
		.args(&["clone", APTOS_CORE_REPO, temp_dir.to_str().unwrap()])
		.status()
		.expect("Failed to clone repository");

	if !status.success() {
		panic!("Failed to clone repository");
	}

	// Checkout specific commit
	let status = Command::new("git")
		.current_dir(&temp_dir)
		.args(&["checkout", APTOS_CORE_COMMIT])
		.status()
		.expect("Failed to checkout commit");

	if !status.success() {
		panic!("Failed to checkout commit");
	}

	// Build the aptos binary
	println!("cargo:warning=Building aptos binary...");
	let status = Command::new("cargo")
		.current_dir(&temp_dir)
		.args(&["build", "-p", "aptos", "--release"])
		.status()
		.expect("Failed to build aptos");

	if !status.success() {
		panic!("Failed to build aptos");
	}

	// Copy the binary to our target location
	let built_binary = temp_dir.join("target/release/aptos");
	if !built_binary.exists() {
		panic!("Built binary not found at expected location");
	}

	fs::copy(&built_binary, &aptos_bin_path).expect("Failed to copy binary");

	// Write the cache file
	fs::write(&cache_path, hash).expect("Failed to write cache file");

	// Clean up the temporary directory
	fs::remove_dir_all(&temp_dir).expect("Failed to clean up temp directory");

	println!("cargo:warning=Aptos binary built and cached successfully");
	println!("cargo:rustc-env=APTOS_BINARY_PATH={}", aptos_bin_path.display());
}
