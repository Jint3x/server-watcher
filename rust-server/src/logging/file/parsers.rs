use crate::parse_config::{Config, LogType, LogCredentials};


pub fn get_directory(config: &Config) -> String {
    match config.log_type {
        LogType::File => {
            match &config.log_credentials {
                LogCredentials::FileLog { path } => path.into(),
                _ => panic!("The logging mode is not set to file logging.")
            }
        },

        _ => panic!("The logging mode is not set to file logging.")
    }
}


#[cfg(test)]
mod tests {
    use crate::parse_config::{Config, ConfigMode, LogType, LogCredentials};
    use super::get_directory;


    #[test]
    fn get_directory_gets_dir() {
        let config = Config {
            mode: ConfigMode::ConfigWarn {
                cpu_limit: 0,
                ram_limit: 0,
                swap_limit: 0,
                disk_limit: 0,
            },
            interval: 10,
            log_type: LogType::File,
            log_credentials: LogCredentials::FileLog {
                path: "C:/special/path".into()
            }
        };

        let path = get_directory(&config);

        assert_eq!(path, "C:/special/path");
    }


    #[test]
    #[should_panic = "The logging mode is not set to file logging."]
    fn get_directory_wrong_logging_mode() {
        let config = Config {
            mode: ConfigMode::ConfigWarn {
                cpu_limit: 0,
                ram_limit: 0,
                swap_limit: 0,
                disk_limit: 0,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "asd".into(),
                channel: 12345,
            }
        };

        get_directory(&config);
    }
}