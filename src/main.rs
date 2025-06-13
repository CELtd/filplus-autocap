use filplus_autocap::runtime::{logging, config, app};

/// Entry point for the Filplus Autocap Allocator.
///
/// Initializes logging, loads configuration, and launches the MasterBot.
fn main() -> anyhow::Result<()> {
    // Initialize logger (console + file output, rotated)
    logging::init_logger()?;

    // Load environment configuration (e.g., RPC URL, file paths)
    let config = config::load_config()?;

    // Start the application loop with loaded configuration
    app::run_app(config)
}
