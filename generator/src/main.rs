use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command line arguments.
    let opts = generator::Opts::parse();

    // Setup our logger.
    let drain = opts.create_logger();
    let logger = slog::Logger::root(drain, slog::o!());

    let _scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init()?;

    // Let's read the spec from the file.
    let spec = generator::load_api(&opts.input).await?;

    // Generate the library.
    generator::generate(&spec, &opts).await?;

    Ok(())
}
