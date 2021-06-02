pub mod parse_config;
pub mod logging;
pub mod metrics;

pub fn start() {
    let config = parse_config::create_config();
    
    logging::file::start(config);
}