#[derive(Debug, PartialEq)]
pub enum ConfigMode {
    ConfigInterval {
        ram: bool,
        cpu: bool,
        system_uptime: bool,
        cpu_average: bool,
        disk: bool,
        swap: bool,
    },

    ConfigWarn {
        cpu_limit: u32,
        ram_limit: u32,
        disk_limit: u32,
        swap_limit: u32,
    }
}

#[derive(Debug, PartialEq)]
pub enum LogCredentials {
    DiscordLog {
        key: String,
        channel: u64,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogType {
    Discord
}

#[derive(Debug, PartialEq)]
pub enum ErrorLogType {
    TypeNonExistent,
}

#[derive(Debug, PartialEq)]
// Contains all information about the current config used in the environment file
pub struct Config {
    // What mode will be used, each mode will be packed with its specific items
    pub mode: ConfigMode,

    // How often should the program run
    pub interval: u32,

    // Where should the metrics be logged
    pub log_type: LogType,

    // The credentials for the logging method (ex: Discord API key, channel id)
    pub log_credentials: LogCredentials
}



pub fn create_config() -> Config {
    let mode = parse_mode();
    let interval = std::env::var("interval")
    .expect("Couldn't find the interval variable")
    .parse::<u32>()
    .expect("Couldn't parse the interval variable to a positive u32 integer");

    let log_type = get_log_type().expect("Wrong type variable specified");
    let log_credentials = parse_credentials(log_type.clone());

    Config {
        mode,
        interval,
        log_type,
        log_credentials
    }
}


// Get the watching mode, it can be either warn or interval.
fn parse_mode() -> ConfigMode {
    let mode = std::env::var("mode").expect("Couldn't parse the mode variable");

    if mode.eq_ignore_ascii_case("warn") {
        get_warn_mode()
    } else {
        get_interval_mode()
    }
}


fn get_warn_mode() -> ConfigMode {
    let ram_limit = std::env::var("ram_limit")
    .expect("ram_limit variable not specified")
    .parse::<u32>()
    .expect("Couldn't parse the ram_limit to a number");

    let cpu_limit = std::env::var("cpu_limit")
    .expect("cpu_limit variable not specified")
    .parse::<u32>()
    .expect("Couldn't parse the cpu_limit to a number");

    let disk_limit = std::env::var("disk_limit")
    .expect("cpu_limit variable not specified")
    .parse::<u32>()
    .expect("Couldn't parse the cpu_limit to a number");

    let swap_limit = std::env::var("swap_limit")
    .expect("cpu_limit variable not specified")
    .parse::<u32>()
    .expect("Couldn't parse the cpu_limit to a number");

    if ram_limit > 100 || cpu_limit > 100 { panic!("The ram/cpu limit cannot exceed 100%") };

    ConfigMode::ConfigWarn {
        ram_limit,
        cpu_limit,
        disk_limit,
        swap_limit
    }
}


fn get_interval_mode() -> ConfigMode {
    ConfigMode::ConfigInterval {
        ram: parse_env_var_to_boolean("ram"),
        cpu: parse_env_var_to_boolean("cpu"),
        cpu_average: parse_env_var_to_boolean("cpu_average"),
        system_uptime: parse_env_var_to_boolean("system_uptime"),
        disk: parse_env_var_to_boolean("disk"),
        swap: parse_env_var_to_boolean("swap"),
    }
}


pub fn parse_env_var_to_boolean(env: &str) -> bool {
    std::env::var(env)
    .unwrap_or("false".to_string())
    .trim()
    .parse::<bool>()
    .expect("Couldn't parse the parameter to a boolean") // Might need a better error message
}


pub fn get_log_type() -> Result<LogType, ErrorLogType> {
    let log = std::env::var("type").expect("Couldn't find a type variable");

    match &*log {
        "discord" => Ok(LogType::Discord),
        _ => Err(ErrorLogType::TypeNonExistent)
    }
}


// Given the logging type, fetch its credentials and return them.
pub fn parse_credentials(log_type: LogType) -> LogCredentials {
    match log_type {
        LogType::Discord => {
            let discord_key = std::env::var("discord_key").expect("Couldn't get the discord_key variable to login");
            let discord_channel = std::env::var("discord_channel").expect("Couldn't get the discord_channel variable");

            LogCredentials::DiscordLog {
                key: discord_key,
                channel: discord_channel.parse::<u64>().expect("Couldn't convert the discord channel to a number")
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::env::{set_var, remove_var};
    use super::{Config, LogCredentials,  ConfigMode, LogType, ErrorLogType};
    use super::{parse_mode, get_log_type, parse_credentials, create_config};


    #[test]
    fn create_config_creates() {
        set_var("mode", "warn");
        set_var("ram_limit", "20");
        set_var("cpu_limit", "20");
        set_var("swap_limit", "15");
        set_var("disk_limit", "10");
        set_var("interval", "10");
        set_var("type", "discord");
        set_var("discord_key", "special_secret_key");
        set_var("discord_channel", "123456789");

        let config = create_config();
        let test_config = Config {
            mode: ConfigMode::ConfigWarn {
                ram_limit: 20,
                cpu_limit: 20,
                disk_limit: 10,
                swap_limit: 15,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "special_secret_key".to_string(),
                channel: 123456789
            }
        };

        assert_eq!(config, test_config)
    }


    #[test]
    fn parse_mode_parses_warn() {
        set_var("mode", "warn");
        set_var("ram_limit", "20");
        set_var("cpu_limit", "20");
        set_var("disk_limit", "10");
        set_var("swap_limit", "5");

        let warn_mode = parse_mode();
        let test_mode = ConfigMode::ConfigWarn { 
            cpu_limit: 20,
            ram_limit: 20,
            disk_limit: 10,
            swap_limit: 5,
        };

        assert_eq!(warn_mode, test_mode);
    }


    #[test]
    #[should_panic = "Couldn't parse the ram_limit to a number"]
    fn parse_mode_panics_not_a_num_warn() {
        set_var("mode", "warn");
        set_var("ram_limit", "will not parse");
        set_var("cpu_limit", "will not parse");
        set_var("disk_limit", "will not parse");
        set_var("swap_limit", "will not parse");

        parse_mode();
    }


    #[test]
    #[should_panic = "ram_limit variable not specified"]
    fn parse_mode_panics_not_set_warn() {
        set_var("mode", "warn");
        set_var("cpu_limit", "20");
        set_var("disk_limit", "20");
        set_var("swap_limit", "20");
        remove_var("ram_limit");

        parse_mode();
    }


    #[test]
    fn parse_mode_parses_interval() {
        set_var("mode", "interval");
        set_var("ram", "true");
        set_var("cpu", "true");
        set_var("cpu_average", "true");
        set_var("system_uptime", "true");
        set_var("disk", "true");
        set_var("swap", "false");

        let interval_mode = parse_mode();
        let test_mode = ConfigMode::ConfigInterval {
            ram: true,
            cpu: true,
            cpu_average: true,
            system_uptime: true,
            disk: true,
            swap: false,
        };

        assert_eq!(interval_mode, test_mode);
    }


    #[test]
    fn get_log_type_gets_discord() {
        set_var("type", "discord");
        let log_type = get_log_type().unwrap();

        assert_eq!(log_type, LogType::Discord)
    }


    #[test]
    #[should_panic = "Couldn't find a type variable"]
    fn get_log_type_no_var() {
        remove_var("type");
        get_log_type().unwrap();
    }


    #[test]
    fn get_log_type_wrong_var() {
        set_var("type", "will_not_exist");
        let log_type = get_log_type();

        match log_type {
            Ok(_) => panic!("Should've been an error"),
            Err(err) => assert_eq!(err, ErrorLogType::TypeNonExistent)
        }
    }


    #[test]
    fn parse_credentials_parses() {
        set_var("discord_key", "my_special_key");
        set_var("discord_channel", "123456789");

        let discord_credentials = parse_credentials(LogType::Discord);
        let custom_discord_credentials = LogCredentials::DiscordLog {
            key: "my_special_key".to_string(),
            channel: 123456789,
        };

        assert_eq!(discord_credentials, custom_discord_credentials);
    }


    #[test]
    #[should_panic = "Couldn't get the discord_key variable to login"]
    fn parse_credentials_no_discord_key() {
        remove_var("discord_key");
        parse_credentials(LogType::Discord);
    }


    #[test]
    #[should_panic = "Couldn't get the discord_channel variable"]
    fn parse_credentials_no_discord_channel() {
        set_var("discord_key", "my_special_key");
        remove_var("discord_channel");
        parse_credentials(LogType::Discord);
    }


    #[test]
    #[should_panic = "Couldn't convert the discord channel to a number"]
    fn parse_credentials_discord_channel_not_a_num() {
        set_var("discord_key", "my_special_key");
        set_var("discord_channel", "will not work");
        parse_credentials(LogType::Discord);
    }
}