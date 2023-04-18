use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    println!("Starting openapitor version {}", env!("CARGO_PKG_VERSION"));

    // Parse the command line arguments.
    let opts = openapitor::Opts::parse();

    // Setup our logger.
    let drain = opts.create_logger();
    let logger = slog::Logger::root(drain, slog::o!());

    slog_scope::set_global_logger(logger).cancel_reset();
    slog_stdlog::init()?;

    // Let's read the spec from the file.
    let spec = openapitor::load_api(&opts.input)?;

    // Generate the library.
    openapitor::generate(&spec, &opts)?;

    Ok(())
}
