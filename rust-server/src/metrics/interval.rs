use super::super::parse_config::{Config, ConfigMode};
use sysinfo::{ProcessorExt, SystemExt};
use super::{get_used_disk_space};

#[derive(Debug, PartialEq)]
pub struct IntervalMetrics {
    // Currently used ram
    pub ram: i64,

    // Currently used CPU
    pub cpu: f32,

    // How long has the host system been running for
    pub system_uptime: i64,

    // CPU average for the past 1, 5 and 15 minutes.
    pub cpu_average: (f64, f64, f64),

    // Currently used disk space [In MB]
    pub disk: i64,

    // Currently used Swap in KB
    pub swap: i64,
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
                ram: -1,
                cpu: -1.0,
                system_uptime: -1,
                cpu_average: (-1.0, -1.0, -1.0),
                disk: -1,
                swap: -1,
            };
        
            // Check each metric, if it is enabled, set it.
            if ram { metrics.ram = system.get_used_memory() as i64 };

            if cpu { metrics.cpu = system.get_global_processor_info().get_cpu_usage() }
            
            if system_uptime { metrics.system_uptime = system.get_uptime() as i64 }

            if disk { metrics.disk = get_used_disk_space(system.get_disks()) }

            if swap { metrics.swap = system.get_used_swap() as i64 }

            if cpu_average { 
                let avg_load = system.get_load_average();
                metrics.cpu_average = (avg_load.one, avg_load.five, avg_load.fifteen);
            }

            metrics
        } else {
            panic!("The passed config mode does not have ConfigInterval as its mode.")
        }
    }


    // Check which metric is enabled and update it. 
    pub fn update_metrics(&mut self, system: &mut sysinfo::System) {
        system.refresh_all();
        
        if self.ram > -1 { self.ram = system.get_used_memory() as i64 }

        if self.cpu > -1.0 { self.cpu = system.get_global_processor_info().get_cpu_usage() }

        if self.system_uptime > -1 { self.system_uptime = system.get_uptime() as i64 }

        if self.disk > -1 { self.disk = get_used_disk_space(system.get_disks()) }

        if self.swap > -1 { self.swap = system.get_used_swap() as i64 } 

        if self.cpu_average.0 > -1.0 {
            let avg_load = system.get_load_average();
            self.cpu_average = (avg_load.one, avg_load.five, avg_load.fifteen);
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

        assert_ne!(metrics.ram, -1);
        assert_ne!(metrics.disk, -1);
        assert_ne!(metrics.cpu, -1.0);
        assert_ne!(metrics.cpu_average, (-1.0, -1.0, -1.0));
        assert_ne!(metrics.system_uptime, -1);
        assert_ne!(metrics.swap, -1);
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

        assert_ne!(metrics.ram, -1);
        assert_ne!(metrics.cpu, -1.0);
        assert_eq!(metrics.cpu_average, (-1.0, -1.0, -1.0));
        assert_eq!(metrics.system_uptime, -1);
        assert_ne!(metrics.disk, -1);
        assert_eq!(metrics.swap, -1);
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