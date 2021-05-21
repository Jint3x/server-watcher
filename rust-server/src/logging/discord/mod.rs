use super::super::parse_config::Config;
mod parsers;
use tokio;
use serenity::{http, model::{id::ChannelId}};




#[tokio::main] // Do a git commit later 
pub async fn start(config: Config) {
    let (token, channel) = parsers::parse_token_and_channel(config);
    let discord_connection = http::Http::new_with_token(&token);
    let channel_connection = ChannelId(channel);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));

        channel_connection.send_message(&discord_connection, |m| {
            m.content("Bot Loaded");
            m
        })
        .await 
        .unwrap();
    }

}