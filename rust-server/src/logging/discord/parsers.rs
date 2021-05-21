use super::super::super::parse_config::{LogType, Config, LogCredentials};

pub fn parse_token_and_channel(config: Config) -> (String, u64) {
    let log_type = config.log_type;
    let log_credentials = config.log_credentials;


    match log_type {
        LogType::Discord => {
            match log_credentials {
                LogCredentials::DiscordLog { key, channel } => {
                    (key, channel)
                },
                _ => panic!("Wrong log credentials are being used")
            }
        },
        _ => panic!("The current logging type is not discord")
    }
}