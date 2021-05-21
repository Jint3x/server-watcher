#[derive(Debug)]
pub enum ConfigMode {
    ConfigInterval {
        ram: bool,
        cpu: bool,
        system_uptime: bool,
        cpu_average: bool
    },

    ConfigWarn {
        cpu_limit: u32,
        ram_limit: u32
    }
}

#[derive(Debug)]
pub enum LogCredentials {
    DiscordLog {
        key: String,
        channel: u64,
    }
}

#[derive(Debug, Clone)]
pub enum LogType {
    Discord
}

#[derive(Debug)]
pub enum ErrorLogType {
    TypeNonExistent,
}

#[derive(Debug)]
// Contains all information about the current config used in the environment file
pub struct Config {
    // What mode it will be, each mode will be packed with its specific items
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

    let log_type = get_log_type().expect("Wronge type variable specified");
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
    .expect("Couldn't parse the ram_limit to a number");

    if ram_limit > 100 || cpu_limit > 100 { panic!("The ram/cpu limit cannot exceed 100%") };

    ConfigMode::ConfigWarn {
        ram_limit,
        cpu_limit
    }
}


fn get_interval_mode() -> ConfigMode {
    ConfigMode::ConfigInterval {
        ram: parse_env_var_to_boolean("ram"),
        cpu: parse_env_var_to_boolean("cpu"),
        cpu_average: parse_env_var_to_boolean("cpu_average"),
        system_uptime: parse_env_var_to_boolean("system_uptime")
    }
}


pub fn parse_env_var_to_boolean(env: &str) -> bool {
    std::env::var(env)
    .unwrap_or("false".to_string())
    .parse::<bool>()
    .unwrap_or(false)
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
    use std::env::set_var;
    use super::ConfigMode;

    use super::parse_mode;

    #[test]
    fn parse_mode_parses_warn() {
        set_var("mode", "warn");
        set_var("ram_limit", "20");
        set_var("cpu_limit", "20");

        let warn_mode = parse_mode();
        let test_mode = ConfigMode::ConfigWarn { cpu_limit: 20, ram_limit: 20 };

        match warn_mode {
            ConfigMode::ConfigWarn { cpu_limit, ram_limit } => {
                
                match test_mode {
                    ConfigMode::ConfigWarn { cpu_limit: cpu_limit_test, ram_limit: ram_limit_test} => {
                        assert_eq!(cpu_limit, cpu_limit_test);
                        assert_eq!(ram_limit, ram_limit_test)
                    },
                    _ => panic!("")
                }

            },
            _ => panic!("")
        }
    }

    #[test]
    fn parse_mode_parses_interval() {
        set_var("mode", "interval");
        set_var("ram", "true");
        set_var("cpu", "true");
        set_var("cpu_average", "true");
        set_var("system_uptime", "true");

        let interval_mode = parse_mode();
        let test_mode = ConfigMode::ConfigInterval {
            ram: true,
            cpu: true,
            cpu_average: true,
            system_uptime: true
        };

        match interval_mode {
            ConfigMode::ConfigInterval { ram, cpu, cpu_average, system_uptime } => {
                match test_mode {
                    ConfigMode::ConfigInterval { cpu: cpu_test, ram: ram_test, cpu_average: cpu_average_test, system_uptime: system_uptime_test} => {
                        assert_eq!(cpu, cpu_test);
                        assert_eq!(ram, ram_test);
                        assert_eq!(cpu_average, cpu_average_test);
                        assert_eq!(system_uptime, system_uptime_test);
                    },
                    _ => panic!("")
                }

            },
            _ => panic!("Not an interval mode")
        }
    }

}