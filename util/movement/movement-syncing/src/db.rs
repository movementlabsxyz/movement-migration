use movement_types::actor;
pub use movement_types::application;
use std::path::PathBuf;
use syncador::PullOperations;
use tempdir::TempDir;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbSync {
	s3_bucket: String,
	destination_db_path: PathBuf,
	application_id: application::Id,
	region: String,
}

impl DbSync {
	const DB_SYNC_PATTERN: &str =
		"{default_signer_address_whitelist,maptos,maptos-storage,movement-da-db}/**";

	/// Create a new db sync instance with a custom s3 bucket and destination db path
	pub fn new(
		s3_bucket: String,
		destination_db_path: PathBuf,
		application_id: application::Id,
		region: String,
	) -> Self {
		Self { s3_bucket, destination_db_path, application_id, region }
	}

	/// Creates a new db sync instance with a `.debug/movement-db-<uid>` destination db path

	pub fn debug(s3_bucket: String, application_id: application::Id, region: String) -> Self {
		let uid = Uuid::new_v4().to_string();

		Self {
			s3_bucket,
			destination_db_path: PathBuf::from(format!(".debug/movement-db-{}", uid)),
			application_id,
			region,
		}
	}

	/// Create a new db sync instance for the mainnet
	pub fn mainnet_debug() -> Self {
		Self::debug(
			"move-main-rec-l-sb-sync".to_string(),
			application::Id::suzuka(),
			"us-west-1".to_string(),
		)
	}

	/// Create a new db sync instance for the mainnet with a temporary destination db path
	pub fn with_tempdir_destination_db_path(self) -> Result<(Self, TempDir), anyhow::Error> {
		let tempdir = TempDir::new("movement-db")?;
		let destination_db_path = tempdir.path().to_path_buf();
		Ok((
			Self {
				s3_bucket: self.s3_bucket,
				destination_db_path,
				application_id: self.application_id,
				region: self.region,
			},
			tempdir,
		))
	}

	/// Borrow the destination db path
	pub fn destination_db_path(&self) -> &PathBuf {
		&self.destination_db_path
	}

	/// Pull the db from the s3 bucket to the destination db path
	pub async fn pull(&self) -> Result<(), anyhow::Error> {
		// set aws region env var to us-east-1
		std::env::set_var("AWS_REGION", self.region.clone()); // this is to avoid having to mess with the syncador config or being too stringent about aws setup

		// Create a new s3 pull instance
		let s3_pull = syncador::backend::s3::shared_bucket::create_pull_with_load_from_env(
			self.s3_bucket.clone(),
			syncador::backend::s3::shared_bucket::metadata::Metadata::default()
				.with_application_id(self.application_id)
				.with_syncer_id(actor::Id::random()), // syncer_id can be anything since we're downsyncing not upsyncing
			self.destination_db_path.clone(),
		)
		.await?;

		// Create a new pull pipeline
		let pull_pipe = syncador::backend::pipeline::pull::Pipeline::new(vec![
			Box::new(s3_pull),
			Box::new(syncador::backend::clear::glob::pull::ClearGlob::try_new(
				&Self::DB_SYNC_PATTERN,
				self.destination_db_path.clone(),
			)?),
			Box::new(syncador::backend::archive::gzip::pull::Pull::new(
				self.destination_db_path.clone(),
			)),
		]);

		// Run the pull
		pull_pipe.pull(Some(syncador::Package::null())).await?;

		Ok(())
	}
}
