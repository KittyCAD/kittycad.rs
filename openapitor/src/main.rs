use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command line arguments.
    let opts = openapitor::Opts::parse();

    // Setup our logger.
    let drain = opts.create_logger();
    let logger = slog::Logger::root(drain, slog::o!());

    slog_scope::set_global_logger(logger).cancel_reset();
    slog_stdlog::init()?;

    // Let's read the spec from the file.
    let spec = openapitor::load_api(&opts.input).await?;

    // Generate the library.
    openapitor::generate(&spec, &opts).await?;

    Ok(())
}
