mod parse_config;
mod logging;

pub fn start() {
    let config = parse_config::create_config();
    
    logging::discord::start(config);
}