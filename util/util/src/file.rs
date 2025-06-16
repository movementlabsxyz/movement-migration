use anyhow::Context;
use std::fs;
use std::path::Path;
use tracing::{debug, info};
use walkdir::WalkDir;

/// Copies a directory recursively.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), anyhow::Error> {
	for entry in WalkDir::new(src) {
		let entry = entry?;
		let rel_path = entry.path().strip_prefix(src).unwrap();
		let dest_path = dst.join(rel_path);

		if entry.file_type().is_dir() {
			let permissions = fs::metadata(&entry.path()).context(format!(
				"failed to get permissions while copying directory recursively from {src:?} {entry:?} to {dst:?}"
			))?;
			fs::create_dir_all(&dest_path).context(format!(
				"failed to create directory while copying directory recursively from {src:?} {entry:?} {permissions:?} to {dst:?}"
			))?;
		} else {
			let permissions = fs::metadata(&entry.path()).context(format!(
				"failed to get permissions while copying file recursively from {src:?} {entry:?} to {dst:?}"
			))?;
			fs::copy(entry.path(), &dest_path).context(format!(
				"failed to copy file while copying directory recursively from {src:?} {entry:?} {permissions:?} to {dst:?}"
			))?;
		}
	}
	Ok(())
}

/// Copies a directory recursively while ignoring specified paths.
pub fn copy_dir_recursive_with_ignore(
	src: &Path,
	ignore_paths: impl IntoIterator<Item = impl AsRef<Path>>,
	dst: &Path,
) -> Result<(), anyhow::Error> {
	let ignore_paths: Vec<_> = ignore_paths.into_iter().collect();
	let mut errors: Vec<Result<(), anyhow::Error>> = Vec::new();

	// Configure WalkDir to be more resilient
	let walker = WalkDir::new(src)
		.follow_links(false)
		.same_file_system(true)
		.into_iter()
		.filter_entry(|entry| {
			// Early filter for ignored paths to avoid permission errors
			let path = entry.path();
			!ignore_paths.iter().any(|ignore| {
				path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref())
			})
		});

	for entry in walker {
		let entry = match entry {
			Ok(e) => e,
			Err(e) => {
				// Check if this is a permission error on an ignored path
				if let Some(path) = e.path() {
					if ignore_paths.iter().any(|ignore| {
						path.to_string_lossy().contains(ignore.as_ref().to_string_lossy().as_ref())
					}) {
						debug!("Ignoring permission error on ignored path: {}", path.display());
						continue;
					}
				}
				// For other errors, log with info for now and fail.
				info!("Error accessing path: {:?}", e);
				errors.push(Err(anyhow::anyhow!("failed to get entry: {:?}", e)));
				continue;
			}
		};

		debug!("Processing entry: {}", entry.path().display());

		let path = entry.path();
		let dest_path = dst.join(path.strip_prefix(src).context("failed to strip prefix")?);

		if entry.file_type().is_dir() {
			match fs::create_dir_all(&dest_path) {
				Ok(_) => (),
				Err(e) => errors.push(Err(e.into())),
			}
		} else {
			if let Some(parent) = dest_path.parent() {
				match fs::create_dir_all(parent) {
					Ok(_) => (),
					Err(e) => errors.push(Err(e.into())),
				}
			}
			match fs::copy(path, &dest_path) {
				Ok(_) => (),
				Err(e) => errors.push(Err(e.into())),
			}
		}
	}

	// Combine all results into one
	if !errors.is_empty() {
		let mut total_error_message =
			String::from("failed to copy directory with the following errors: ");
		for error in errors {
			total_error_message = total_error_message + &format!("{:?}\n", error);
		}
		return Err(anyhow::anyhow!(total_error_message));
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
	use std::fs::Permissions;
	use std::os::unix::fs::PermissionsExt;
	use std::path::PathBuf;
	use tempfile::TempDir;

	#[test]
	#[tracing_test::traced_test]
	fn test_copy_dir_recursive_with_ignore() -> Result<(), anyhow::Error> {
		// put some file in a temp dir
		let temp_dir = TempDir::new()?;

		// write a file that should be copied
		let file_path = temp_dir.path().join("maptos").join("test_file.txt");
		fs::create_dir_all(
			file_path
				.parent()
				.context("failed to get parent directory for file that should be copied")?,
		)
		.context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should be copied")?;

		// write a file that should not be copied
		let file_path = temp_dir.path().join("celestia").join("test_file2.txt");
		fs::create_dir_all(
			file_path
				.parent()
				.context("failed to get parent directory for file that should not be copied")?,
		)
		.context("failed to create directory")?;
		fs::write(file_path, "test").context("failed to write file that should not be copied")?;

		// create the target temp dir
		let dst = TempDir::new()?;

		// copy the file to a new dir, ignoring celestia directory
		copy_dir_recursive_with_ignore(&temp_dir.path(), [PathBuf::from("celestia")], &dst.path())
			.context("failed to copy directory")?;

		// check that the file was copied
		assert!(dst.path().join("maptos").join("test_file.txt").exists());

		// check that the celestia directory was not copied
		assert!(!dst.path().join("celestia").join("test_file2.txt").exists());

		Ok(())
	}

	// Somehow the following test is failing on CI: https://github.com/movementlabsxyz/movement-migration/actions/runs/15386989846/job/43287594343
	// This indicates that failure is not due to the inability to ignore copy, but rather some issue performing an oepration that requires permissions.
	#[test]
	fn test_are_you_kidding_me() -> Result<(), anyhow::Error> {
		let source_dir = TempDir::new()?;
		let target_dir = TempDir::new()?;

		let path_that_must_be_ignored = source_dir
			.path()
			.join(".movement/celestia/c1860ae680eb2d91927b/.celestia-app/keyring-test");

		fs::create_dir_all(
			path_that_must_be_ignored
				.parent()
				.context("failed to get parent directory for path that must be ignored")?,
		)
		.context("failed to create directory")?;
		// write a file that must not be ignored
		fs::write(path_that_must_be_ignored.clone(), "test")
			.context("failed to write file that must not be ignored")?;
		// set permissions to 000 on the file and then on the parent directory
		fs::set_permissions(path_that_must_be_ignored.clone(), Permissions::from_mode(0o000))
			.context("failed to set permissions on file that must not be ignored")?;
		fs::set_permissions(
			path_that_must_be_ignored
				.parent()
				.context("failed to get parent directory for path that must be ignored")?,
			Permissions::from_mode(0o000),
		)
		.context("failed to set permissions on parent directory that must not be ignored")?;

		copy_dir_recursive_with_ignore(source_dir.path(), ["celestia"], target_dir.path())
			.context("failed to copy directory")?;

		assert!(!target_dir
			.path()
			.join("celestia")
			.join("c1860ae680eb2d91927b")
			.join(".celestia-app")
			.join("keyring-test")
			.exists());

		Ok(())
	}
}
