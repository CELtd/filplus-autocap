use filplus_autocap::runtime::{logging, config, app};

fn main() -> anyhow::Result<()> {
    logging::init_logger()?;
    let config = config::load_config()?;
    app::run_app(config)
}
