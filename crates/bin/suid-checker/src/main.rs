mod error;

pub use self::error::{Error, Result};
use lib_checker::checker::SuidChecker;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting SUID binary scan...");

    let checker = SuidChecker::new();

    checker.is_linux()?;
    checker.check_suid_binaries()?;

    info!("SUID binary scan completed.");

    Ok(())
}
