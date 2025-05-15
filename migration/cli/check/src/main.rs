use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::*;
use dotenv::dotenv;
use mtma_check::cli;
use tracing_subscriber::EnvFilter;

#[tokio::main]
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
	let mtma_check = cli::MtmaCheck::parse();
	mtma_check.execute().await?;
	Ok(())
}
