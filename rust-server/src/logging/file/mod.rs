use crate::parse_config::{ Config, ConfigMode };
use crate::metrics::{interval::IntervalMetrics, warn::WarnMetrics};
use sysinfo::{self, System, SystemExt};
use std::fs;
use std::path::PathBuf;
use std::io::Write;


mod parsers;


pub fn start(config: Config) {
    let directory = parsers::get_directory(&config);
    let system = sysinfo::System::new_all();

    match config.mode {
        ConfigMode::ConfigInterval {
            ram: _,
            cpu: _,
            cpu_average: _,
            system_uptime: _,
            disk: _,
            swap: _
        } => {
            let metrics = IntervalMetrics::new(&config, &system);
            log_interval(
                directory,
                system,
                metrics, 
                config
            )
        },

        ConfigMode::ConfigWarn {
            cpu_limit: _,
            ram_limit: _,
            disk_limit: _,
            swap_limit: _,
        } => {
            let metrics = WarnMetrics::new(&config);
            log_warn(
                directory,
                system,
                metrics,
                config
            )
        }
    }
}


fn log_interval(
    dir: String,
    mut system: System, 
    mut metrics: IntervalMetrics,
    config: Config,
) {
    // Create the logging path if it does not exist.
    fs::create_dir_all(&dir).expect("Couldn't create the specified logging directory");
    let base_file = "log";

    loop {
        std::thread::sleep(std::time::Duration::from_secs(config.interval as u64));

        let next_name = find_next_free_name(base_file, &dir);
        let metrics_formatted = format_interval_metrics_text(&metrics, &system);
        let file_path = PathBuf::from(&dir).join(next_name);
        let mut file = fs::File::create(file_path).expect("Couldn't create a file at the expected logging directory");

        file.write_all(metrics_formatted.as_bytes()).expect("Couldn't write information to a logging file");
        metrics.update_metrics(&mut system);
    }
}


// Loop through all the metrics, check if they are enabled and if they are combine
// them into a string and return them
fn format_interval_metrics_text(metrics: &IntervalMetrics, system: &System) -> String {
    let mut metric_list = Vec::new();

    if metrics.ram.is_some() {
        let message = format!("Used RAM: {} MB out of {} MB", metrics.ram.unwrap() / 1000, system.get_total_memory() / 1000);
        metric_list.push(message)
    }

    if metrics.swap.is_some() {
        let message = format!("Used Swap: {} MB out of {} MB", metrics.swap.unwrap() / 1000, system.get_total_swap() / 1000);
        metric_list.push(message)
    }

    if metrics.cpu.is_some() {
        let message = format!("Used CPU: {:.2}", metrics.cpu.unwrap());
        metric_list.push(message); 
    }

    if metrics.cpu_average.is_some() {
        let (min, five_min, fteen_min) = metrics.cpu_average.unwrap();
        let message = format!("CPU Average: 1 minute - {:.2}, 5 minutes - {:.2}, 15 minutes - {:.2}", min, five_min, fteen_min);
        metric_list.push(message)
    }

    if metrics.system_uptime.is_some() {
        let message = format!("System Uptime: {} minutes", metrics.system_uptime.unwrap() / 1000);
        metric_list.push(message)
    }

    if metrics.disk.is_some() {
        let message = format!("Used Disk Space: {} MB", metrics.disk.unwrap() / 1000);
        metric_list.push(message)
    }

    metric_list.join("\n")
}


fn log_warn(
    dir: String, 
    system: System, 
    metrics: WarnMetrics,
    config: Config,
) {
    /*
        Algorithm:
        1: Create/Check for the path/warn folder
        2: Loop through all files in the directory, find the biggest 'log' file, like a number: 
        log_1, log_2, log_3, .... until the last log file, then the new element would be log_n+1
        3: Extract all possible warnings and format them in a variable.
        4: Save the message.
    */
}


// Opens a directory and finds the next free level of the passed file name, extended with _n.
// Ex: in a directory, there're files log_1, log_2, log_3. This function will return log_4.
fn find_next_free_name(filename: &str, dir: &str) -> String {
    let dir_items = fs::read_dir(dir).expect("Couldn't read the specified logging directory");
    let mut new_name = String::from("");
    let mut num_of_files = 1;
    let mut dir_files = std::collections::HashMap::new();


    // Save each file name into a hashmap which will later be queried
    dir_items.for_each(|file| {
        let file_from_dir = file.unwrap().file_name().to_str().unwrap().to_string();
        dir_files.insert(file_from_dir, 1);
    });

    // Loop and increment the name until a non taken file name spot is found
    loop {
        new_name = String::from(format!("{}_{}.txt", filename, num_of_files));
        if dir_files.get(&new_name).is_none() { break }

        num_of_files += 1;
    }

    new_name
}


#[cfg(test)]
mod tests {
    use std::{fs};
    use sysinfo::{SystemExt, System};
    use crate::metrics::{interval::IntervalMetrics};
    use crate::parse_config::{Config, ConfigMode, LogType, LogCredentials};

    use super::{find_next_free_name, format_interval_metrics_text};


    #[test]
    fn find_next_free_name_finds_a_name() {
        let dir = std::env::current_dir().unwrap().join("TEST");

        fs::create_dir(&dir).unwrap();
        fs::File::create("./TEST/log_2.txt").unwrap();
        fs::File::create("./TEST/log_1.txt").unwrap();

        let name = find_next_free_name("log", dir.to_str().unwrap());
        
        // Careful when editing this test to not accidently delete your files.
        fs::remove_dir_all(&dir).unwrap();
        

        assert_eq!(name, "log_3.txt")
    }


    #[test]
    fn format_interval_metrics_text_formats_some() {
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: true,
                cpu: true,
                cpu_average: false,
                system_uptime: false,
                disk: false,
                swap: true,
            },
            interval: 10,
            log_type: LogType::File,
            log_credentials: LogCredentials::FileLog {
                path: "C:/random/path".into()
            }
        };
        let system = System::new_all();
        let metrics = IntervalMetrics::new(&config, &system);

        let message = format_interval_metrics_text(&metrics, &system);

        let separated_metrics = message.lines().collect::<Vec<&str>>();
        let metric_messages = vec!["Used RAM:", "Used Swap:", "Used CPU:"];

        assert_eq!(separated_metrics.len(), 3);

        for (ind, &value) in separated_metrics.iter().enumerate() {
            println!("{} {}", value, metric_messages[ind]);
            assert_eq!(value.starts_with(metric_messages[ind]), true);
        }
    }


    #[test]
    fn format_interval_metrics_text_formats_all() {
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: true,
                cpu: true,
                cpu_average: true,
                system_uptime: true,
                disk: true,
                swap: true,
            },
            interval: 10,
            log_type: LogType::File,
            log_credentials: LogCredentials::FileLog {
                path: "C:/random/path".into()
            }
        };
        let system = System::new_all();
        let metrics = IntervalMetrics::new(&config, &system);

        let message = format_interval_metrics_text(&metrics, &system);

        let separated_metrics = message.lines().collect::<Vec<&str>>();
        let metric_messages = vec!["Used RAM:", "Used Swap:", "Used CPU:", 
                                            "CPU Average:", "System Uptime:", "Used Disk Space:"];

        assert_eq!(separated_metrics.len(), 6);

        for (ind, &value) in separated_metrics.iter().enumerate() {
            assert_eq!(value.starts_with(metric_messages[ind]), true);
        }
    }
}
