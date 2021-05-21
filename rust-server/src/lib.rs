mod parse_config;

pub fn start() {
    let config = parse_config::create_config();

    println!("{:?}", config);
}