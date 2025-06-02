use anyhow::Result;
use flexi_logger::{Logger, Duplicate, FileSpec};

pub fn init_logger() -> Result<()> {
    Logger::try_with_str("info")?
        .duplicate_to_stdout(Duplicate::Info)
        .log_to_file(FileSpec::default().directory("logs"))
        .rotate(
            flexi_logger::Criterion::Size(10_000_000),
            flexi_logger::Naming::Numbers,
            flexi_logger::Cleanup::KeepLogFiles(5),
        )
        .start()?;

    log::info!("📓 Logger initialized.");
    Ok(())
}
