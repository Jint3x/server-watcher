
enum ConfigMode {
    ConfigInterval {
        ram: bool,
        cpu: bool,
        system_uptime: bool,
        average_cpu: bool
    },

    ConfigWarn {
        cpu_limit: u32,
        ram_limit: u32
    }
}

enum LogCredentials {
    DiscordLog {
        secret: String,
        channel_id: String,
    }
}

// Contains all information about the current config used in the environment file
pub struct Config {
    // What mode it will be, each mode will be packed with its specific items
    mode: ConfigMode,

    // How often should the program run
    interval: u32,

    // Where should the metrics be logged
    log_type: String,

    // The credentials for the logging method (ex: Discord API key, channel id)
    log_credentials: LogCredentials
}



pub fn create_config() -> Config {
    Config {
        mode: ConfigMode::ConfigWarn { cpu_limit: 23, ram_limit: 23 },
        interval: 25,
        log_type: String::from("discord"),
        log_credentials: LogCredentials::DiscordLog { secret: "asd".to_string(), channel_id: "asd".to_string() }
    }
}

