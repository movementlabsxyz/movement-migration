use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::*;
use dotenv::dotenv;
use movement_aptos::cli;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
	// Load environment variables from .env file.
	dotenv().ok();

	// initialize tracing env filter (no logs by default)
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("off")),
		)
		.init();

	// Run the CLI.
	let movement_aptos = cli::MovementAptos::parse();
	movement_aptos.execute().await?;
	Ok(())
}
