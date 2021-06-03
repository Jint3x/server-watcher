use crate::parse_config::{ Config, ConfigMode };
use crate::metrics::{interval::IntervalMetrics, warn::WarnMetrics};
use sysinfo::{self, System, SystemExt};
use std::fmt::format;
use std::fs;


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
            log_interval(directory, system, metrics)
        },

        ConfigMode::ConfigWarn {
            cpu_limit: _,
            ram_limit: _,
            disk_limit: _,
            swap_limit: _,
        } => {
            let metrics = WarnMetrics::new(&config);
            log_warn(directory, system, metrics)
        }
    }
}


fn log_interval(dir: String, system: System, metrics: IntervalMetrics) {
    // Create the logging path if it does not exist.
    fs::create_dir_all(&dir).expect("Couldn't create the specified logging directory");

    let base_file = "log";
    let next_name = find_highest_name(base_file, &dir);
    
    /*
        Algorithm:
        1: Create/Check for the path/interval folder
        2: Loop through all files in the directory, find the biggest 'log' file, like a number: 
        log_1, log_2, log_3, .... until the last log file, then the new element would be log_n+1
        3: Extract all the metrics, save them in a formatted variable.
        4: Save the message.
    */

}


fn log_warn(dir: String, system: System, metrics: WarnMetrics) {
    
}


// Opens a directory and finds the next free level of the passed file name, extended with _n.
// Ex: in a directory, there're files log_1, log_2, log_3. This function will return log_4.
fn find_highest_name(filename: &str, dir: &str) -> String {
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
    use super::{find_highest_name};


    #[test]
    fn find_highest_name_finds_a_name() {
        let dir = std::env::current_dir().unwrap().join("TEST");

        fs::create_dir(&dir).unwrap();
        fs::File::create("./TEST/log_2.txt").unwrap();
        fs::File::create("./TEST/log_1.txt").unwrap();

        let name = find_highest_name("log", dir.to_str().unwrap());
        
        // Careful when editing this test to not accidently delete your files.
        fs::remove_dir_all(&dir).unwrap();
        

        assert_eq!(name, "log_3.txt")
    }
}
