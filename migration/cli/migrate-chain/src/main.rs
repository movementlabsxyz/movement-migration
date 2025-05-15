use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::*;
use dotenv::dotenv;
use mtma_migrate_chain::cli;
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
	let mtma_migrate_chain = cli::MtmaMigrateChain::parse();
	mtma_migrate_chain.execute().await?;
	Ok(())
}
