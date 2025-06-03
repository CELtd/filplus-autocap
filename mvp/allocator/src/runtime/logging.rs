use anyhow::Result;
use flexi_logger::{Logger, Duplicate, FileSpec};

/// Initializes the global logger with both file and stdout output.
///
/// - Logs are stored in the `logs/` directory.
/// - Files are rotated when they exceed 10 MB.
/// - Keeps up to 5 rotated log files.
/// - Duplicates logs to stdout for real-time visibility.
pub fn init_logger() -> Result<()> {
    Logger::try_with_str("info")?                          // Set default log level
        .duplicate_to_stdout(Duplicate::Info)              // Also print Info+ logs to stdout
        .log_to_file(FileSpec::default().directory("logs"))// Write logs to logs/
        .rotate(
            flexi_logger::Criterion::Size(10_000_000),     // Rotate if file > 10 MB
            flexi_logger::Naming::Numbers,                 // Name files with numbers
            flexi_logger::Cleanup::KeepLogFiles(5),        // Keep last 5 files
        )
        .start()?;                                          // Start the logger

    log::info!("📓 Logger initialized.");
    Ok(())
}
