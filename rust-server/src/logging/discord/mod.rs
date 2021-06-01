use serenity::{
    builder::CreateEmbed,
    http::{self, Http},
    model::{id::ChannelId}
};


use super::super::metrics::{
    interval::IntervalMetrics,
    warn::{WarnMetrics, Warn},
};


use sysinfo::{System, SystemExt};
use super::super::parse_config::{Config, ConfigMode};


use tokio::{self};
mod parsers;


#[tokio::main]
pub async fn start(config: Config) {
    let (token, channel) = parsers::parse_token_and_channel(&config);
    let system = System::new_all();
    let discord_connection = http::Http::new_with_token(&token);
    let channel_connection = ChannelId(channel);


    match config.mode {
        ConfigMode::ConfigInterval {
            ram: _ram,
            cpu: _cpu,
            cpu_average: _cpu_average,
            system_uptime: _system_uptime,
            disk: _disk,
            swap: _swap,
        } => {
            send_interval_message(
          IntervalMetrics::new(&config, &system),
                 system,
                 discord_connection, 
   channel_connection, 
                 config
            )
            .await
        },
        ConfigMode::ConfigWarn {
            cpu_limit: _cpu_limit,
            ram_limit: _ram_limit,
            disk_limit: _disk_limit,
            swap_limit: _swap_limit
        } => {
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
                emb.color((0, 190, 219));
                load_interval_embed(emb, &metrics, &system);
                emb
            });

            msg
        })
        .await
        .expect("Couldn't send a message to this channel");
    }
}


// Check each interval metric. If it's above -1 (enabled), append in to the embed.
fn load_interval_embed(embed: &mut CreateEmbed, metrics: &IntervalMetrics, system: &System) {
    if metrics.ram.is_some() {
        embed.field("Used RAM", format!("{} MB out of {} MB", metrics.ram.unwrap() / 1000, system.get_total_memory() / 1000), false);
    };

    if metrics.swap.is_some() {
        embed.field("Used Swap", format!("{} MB out of {} MB", metrics.swap.unwrap() / 1000, system.get_total_swap() / 1000), false);
    }

    if metrics.cpu.is_some() {
        embed.field("Used CPU", format!("{:.2}%", metrics.cpu.unwrap()), false);
    }

    if metrics.system_uptime.is_some() {
        embed.field("System Uptime", format!("{} minutes", metrics.system_uptime.unwrap() / 60), false);
    }

    if metrics.disk.is_some() {
        embed.field("Used System Space:", format!("{} MB", metrics.disk.unwrap()), false);
    }

    if metrics.cpu_average.is_some() {
        let cpu_avg = metrics.cpu_average.unwrap();

        embed.field(
            "Average Load",
            format!(
                    "One minute: {:.2}%, Five minutes: {:.2}%, Fiveteen minutes: {:.2}%", 
                    cpu_avg.0, cpu_avg.1, cpu_avg.2
                  ),
                 false
        );
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
                emb.color((197, 0, 0));
                load_warn_embed(emb, &metrics, &system);
                emb
            });

            msg
        })
        .await
        .expect("Couldn't send a message to this channel");
    }
}


// Check each warn metric, create a separate embed field for it and set it on the embed.
fn load_warn_embed(embed: &mut CreateEmbed, metrics: &WarnMetrics, system: &System) {
    embed.fields(
        metrics.warnings.iter().map(|metric| {
            match metric {
                &Warn::HighCPU(cpu) => ("CPU Limit Surpassed", format!("{:.2}%", cpu), false),
                &Warn::HighRAM(ram) => ("RAM Limit Surpassed", format!("{:.2}% out of {} MB", ram, system.get_total_memory() / 1000), false),
                &Warn::HighDisk(disk) => ("Disk Limit Surpassed", format!("{:.2}%", disk), false),
                &Warn::HighSwap(swap) => ("Swap Limit Surpassed", format!("{:.2}% out of {} MB", swap, system.get_total_swap() / 1000), false),
            }
        })
    );
}