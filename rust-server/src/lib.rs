pub mod parse_config;
pub mod logging;
pub mod metrics;

use crate::parse_config::{ConfigMode};

pub fn start() {
    let config = parse_config::create_config();

    match config.mode {
        ConfigMode::ConfigInterval { ram: _, cpu: _, cpu_average: _, swap: _, system_uptime: _, disk: _} => {
            logging::discord::start(config);
        }

        ConfigMode::ConfigWarn { cpu_limit: _, ram_limit: _, disk_limit: _, swap_limit: _ } => {
            logging::file::start(config)
        }
    }
}