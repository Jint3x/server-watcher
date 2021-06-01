use super::super::parse_config::{Config, ConfigMode};
use sysinfo::{ProcessorExt, SystemExt};
use super::{get_used_disk_space};

#[derive(Debug, PartialEq)]
pub struct IntervalMetrics {
    // Currently used ram
    pub ram: Option<u64>,

    // Currently used CPU
    pub cpu: Option<f32>,

    // How long has the host system been running for
    pub system_uptime: Option<u64>,

    // CPU average for the past 1, 5 and 15 minutes.
    pub cpu_average: Option<(f64, f64, f64)>,

    // Currently used disk space [In MB]
    pub disk: Option<u64>,

    // Currently used Swap in KB
    pub swap: Option<u64>,
}



impl IntervalMetrics {
    pub fn new(config: &Config, system: &sysinfo::System) -> IntervalMetrics {
        if let ConfigMode::ConfigInterval {
            ram,
            cpu, 
            system_uptime, 
            cpu_average,
            disk,
            swap,
        } = config.mode {
            // Create a struct which will be filled with actual values only if the passed config has them enabled
            // -1 is used as an error (false) code.
            let mut metrics = IntervalMetrics { 
                ram: None,
                cpu: None,
                system_uptime: None,
                cpu_average: None,
                disk: None,
                swap: None,
            };
        
            // Check each metric, if it is enabled, set it.
            if ram { metrics.ram = Some(system.get_used_memory() as u64) };

            if cpu { metrics.cpu = Some(system.get_global_processor_info().get_cpu_usage()) }
            
            if system_uptime { metrics.system_uptime = Some(system.get_uptime()) }

            if disk { metrics.disk = Some(get_used_disk_space(system.get_disks()) as u64) }

            if swap { metrics.swap = Some(system.get_used_swap()) }

            if cpu_average { 
                let avg_load = system.get_load_average();
                metrics.cpu_average = Some((avg_load.one, avg_load.five, avg_load.fifteen));
            }

            metrics
        } else {
            panic!("The passed config mode does not have ConfigInterval as its mode.")
        }
    }


    // Check which metric is enabled and update it. 
    pub fn update_metrics(&mut self, system: &mut sysinfo::System) {
        system.refresh_all();
        
        if self.ram.is_some() { self.ram = Some(system.get_used_memory()) }

        if self.cpu.is_some() { self.cpu = Some(system.get_global_processor_info().get_cpu_usage()) }

        if self.system_uptime.is_some() { self.system_uptime = Some(system.get_uptime()) }

        if self.disk.is_some() { self.disk = Some(get_used_disk_space(system.get_disks()) as u64 ) }

        if self.swap.is_some() { self.swap = Some(system.get_used_swap()) } 

        if self.cpu_average.is_some() {
            let avg_load = system.get_load_average();
            self.cpu_average = Some((avg_load.one, avg_load.five, avg_load.fifteen));
        }
    } 
}


#[cfg(test)]
mod tests {
    use super::super::super::parse_config::{Config, ConfigMode, LogType, LogCredentials};
    use super::{IntervalMetrics};
    use sysinfo::{self, SystemExt};


    #[test]
    pub fn intervalmetrics_new_creates() {
        let system = sysinfo::System::new_all();
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
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let metrics = IntervalMetrics::new(&config, &system);

        assert_eq!(metrics.ram.is_some(), true);
        assert_eq!(metrics.disk.is_some(), true);
        assert_eq!(metrics.cpu.is_some(), true);
        assert_eq!(metrics.cpu_average.is_some(), true);
        assert_eq!(metrics.system_uptime.is_some(), true);
        assert_eq!(metrics.swap.is_some(), true);
    }


    #[test]
    #[should_panic = "The passed config mode does not have ConfigInterval as its mode."]
    fn intervalmetrics_new_wrong_mode() {
        let system = sysinfo::System::new_all();
        let config = Config {
            mode: ConfigMode::ConfigWarn {
                cpu_limit: 20,
                ram_limit: 20,
                disk_limit: 20,
                swap_limit: 15,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        IntervalMetrics::new(&config, &system);
    }


    #[test]
    fn intervalmetrics_partly_enabled() {
        let system = sysinfo::System::new_all();
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: true,
                cpu: true,
                cpu_average: false,
                system_uptime: false,
                disk: true,
                swap: false,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let metrics = IntervalMetrics::new(&config, &system);

        assert_eq!(metrics.ram.is_some(), true);
        assert_eq!(metrics.cpu.is_some(), true);
        assert_eq!(metrics.disk.is_some(), true);
        assert_eq!(metrics.cpu_average.is_none(), true);
        assert_eq!(metrics.system_uptime.is_none(), true);
        assert_eq!(metrics.swap.is_none(), true);
    }


    // This test might not be the best one possible. Make sure to review it 
    // and perhaps change it with a better way of chaking the metrics updates.
    #[test]
    fn intervalmetrics_update_metrics_updates() {
        let mut system = sysinfo::System::new_all();
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: false,
                cpu: false,
                cpu_average: false,
                system_uptime: true,
                disk: false,
                swap: false,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let mut metrics = IntervalMetrics::new(&config, &system);
        let old_uptime = metrics.system_uptime.clone();
        std::thread::sleep(std::time::Duration::new(1, 0));
        metrics.update_metrics(&mut system);


        assert_ne!(old_uptime, metrics.system_uptime);
    }
}