use std::process::exit;
use uuid::Uuid;
use crate::config::Config;

mod windows;
mod fs;
mod config;
mod actions;

fn main() {
    colog::init();

    log::info!("Sake {}\n", env!("CARGO_PKG_VERSION"));

    match dotenv::dotenv() {
        Ok(_) => {},
        Err(_) => {
            println!("Couldn't find .env file, assuming it's not a devenv")
        }
    }

    let lock = std::env::var("SAKE_DISABLE_LOCK").unwrap_or("false".into()) == "false";

    if lock {
        fs::acquire_lock(Uuid::new_v4());
    }

    let config: Config = serde_json::from_str(
        &fs::read_config()
    ).unwrap_or_else(|e| {
        log::error!("An error happened during parsing the config.json: {:?}", e);
        exit(0)
    });

    log::info!("Running Sake in `{}` mode\n", config.default_mode);

    let mode = config.modes.get(&config.default_mode).unwrap_or_else(|| {
        log::error!("Unable to run mode `{}`, as it does not exist.", &config.default_mode);
        exit(0)
    });

    fs::temp_operations(&mode, &config);

    fs::copy_packs(&config.meta);

    fs::clear_temp();

    fs::clear_lock()
}
