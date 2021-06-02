use crate::parse_config::{ Config, ConfigMode };
use crate::metrics::{interval::IntervalMetrics, warn::WarnMetrics};
use sysinfo::{self, System, SystemExt};


mod parsers;


pub fn start(config: Config) {
    let directory = parsers::get_directory(&config);
    let system = sysinfo::System::new_all();

    match config.mode {
        ConfigMode::ConfigInterval {
            ram: _,
            cpu: _,
            cpu_average: _,
            system_uptime: _,
            disk: _,
            swap: _
        } => {
            let metrics = IntervalMetrics::new(&config, &system);
            log_interval(directory, system, metrics)
        },

        ConfigMode::ConfigWarn {
            cpu_limit: _,
            ram_limit: _,
            disk_limit: _,
            swap_limit: _,
        } => {
            let metrics = WarnMetrics::new(&config);
            log_warn(directory, system, metrics)
        }
    }
}


fn log_interval(dir: String, system: System, metrics: IntervalMetrics) {

}


fn log_warn(dir: String, system: System, metrics: WarnMetrics) {
    
}