use super::super::super::parse_config::{LogType, Config, LogCredentials, ConfigMode};

pub fn parse_token_and_channel(config: Config) -> (String, u64) {
    let log_type = config.log_type;
    let log_credentials = config.log_credentials;

    if log_type != LogType::Discord { panic!("The current logging mode is not set to discord") }

    match log_credentials {
        LogCredentials::DiscordLog { key, channel } => {
            (key, channel)
        },
        _ => panic!("Wrong log credentials are being used")
    }
}


#[cfg(test)]
mod tests {
    use super::{LogType, Config, LogCredentials, ConfigMode, parse_token_and_channel};

    #[test]
    fn parse_token_channel_parses() {
        let dummy_config = Config {
            mode: ConfigMode::ConfigWarn {
                cpu_limit: 0,
                ram_limit: 0,
            },
            interval: 0,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "test".to_string(),
                channel: 098823098234,
            }
        };

        let (key, channel) = parse_token_and_channel(dummy_config);

        assert_eq!(key, "test".to_string());
        assert_eq!(channel, 098823098234);
    }
}