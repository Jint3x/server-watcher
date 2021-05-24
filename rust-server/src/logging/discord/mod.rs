use serenity::{
    http::{self, Http},
    model::{id::ChannelId}
};

use super::super::metrics::{
    interval::IntervalMetrics,
    warn::WarnMetrics, warn::Warn
};

use sysinfo::{System, SystemExt};
use super::super::parse_config::{Config, ConfigMode};

use tokio;
mod parsers;






#[tokio::main] // Do a git commit later 
pub async fn start(config: Config) {
    let (token, channel) = parsers::parse_token_and_channel(&config);
    let system = System::new_all();
    let discord_connection = http::Http::new_with_token(&token);
    let channel_connection = ChannelId(channel);



    match config.mode {
        ConfigMode::ConfigInterval {ram: _ram, cpu: _cpu, cpu_average: _cpu_average, system_uptime: _system_uptime} => {
            send_interval_message(
          IntervalMetrics::new(&config, &system),
                 system,
                 discord_connection, 
   channel_connection, 
                 config
            )
            .await
        },
        ConfigMode::ConfigWarn {cpu_limit: _cpu_limit, ram_limit: _ram_limit} => {
            send_warn_message(
        WarnMetrics::new(&config), 
                system, 
                discord_connection, 
  channel_connection, 
                config
            )
            .await
        }
    };
}



async fn send_interval_message(
    mut metrics: IntervalMetrics,
    mut system: System, 
    discord_connection: Http, 
    discord_channel: ChannelId,
    config: Config,
) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(config.interval as u64));
        metrics.update_metrics(&mut system);

        discord_channel.send_message(&discord_connection, |msg| {
            msg.embed(|emb| {
                emb.title("Server Interval Metrics");

                // Check for each metric, if it is bigger than -1 (disabled), add it to the embed
                if metrics.ram > -1 {
                    emb.field("Used RAM", format!("{} MB", metrics.ram / 1000), false);
                };

                if metrics.cpu > -1.0 {
                    emb.field("Used CPU", format!("{}%", metrics.cpu), false);
                }

                if metrics.system_uptime > -1 {
                    emb.field("System Uptime", format!("{} minutes", metrics.system_uptime / 60), false);
                }

                if metrics.cpu_average.0 > -1.0 {
                    emb.field(
                        "Average Load",
                        format!(
                                "One minute: {}%, Five minutes: {}%, Fiveteen minutes: {}%", 
                                metrics.cpu_average.0, metrics.cpu_average.1, metrics.cpu_average.2
                              ),
                             false
                    );
                }

                emb
            });

            msg
        })
        .await
        .expect("Couldn't send a message to this channel");
    }
}



async fn send_warn_message(
    mut metrics: WarnMetrics,
    mut system: System, 
    discord_connection: Http,
    discord_channel: ChannelId,
    config: Config,
) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(config.interval as u64));
        metrics.update_warns(&mut system);
        if metrics.warnings.len() < 1 { continue };

        discord_channel.send_message(&discord_connection, |msg| {
            msg.embed(|emb| {
                emb.title("Server Warn Metrics");
                
                emb.fields(
                    metrics.warnings.iter().map(|metric| {
                        match metric {
                            &Warn::HighCPU(cpu) => ("CPU Limit Surpassed", format!("{}%", cpu), false),
                            &Warn::HighRAM(ram) => ("RAM Limit Surpassed", format!("{}%", ram), false)
                        }
                    })
                );

                emb
            });

            msg
        })
        .await
        .expect("Couldn't send a message to this channel");
    }
}