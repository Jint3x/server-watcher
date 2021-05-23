use super::super::parse_config::{Config, ConfigMode};
use sysinfo::{ProcessorExt, SystemExt};

#[derive(Debug, PartialEq)]
pub struct IntervalMetrics {
    // Currently used ram
    pub ram: i64,

    // Currently used CPU
    pub cpu: f32,

    // How long has the host system been running for
    pub system_uptime: i64,

    // CPU average for the past 1, 5 and 15 minutes.
    pub cpu_average: (f64, f64, f64)
}



impl IntervalMetrics {
    pub fn new(config: Config) -> IntervalMetrics {
        if let ConfigMode::ConfigInterval {ram, cpu, system_uptime, cpu_average } = config.mode {
            let system = sysinfo::System::new_all();
            let mut metrics = IntervalMetrics { ram: -1, cpu: -1.0, system_uptime: -1, cpu_average: (-1.0, -1.0, -1.0) };
        
            // Check each metric, if it is enabled, set it.
            if ram { metrics.ram = system.get_used_memory() as i64 };
            if cpu { metrics.cpu = system.get_global_processor_info().get_cpu_usage() }
            if system_uptime { metrics.system_uptime = system.get_uptime() as i64 }
            if cpu_average { 
                let avg_load = system.get_load_average();
                metrics.cpu_average = (avg_load.one, avg_load.five, avg_load.fifteen);
            }

            metrics
        } else {
            panic!("The passed config mode does not have ConfigInterval as its mode.")
        }
    }

    pub fn update_metrics(&mut self) {
        let system = sysinfo::System::new_all();
        
        if self.ram > -1 { self.ram = system.get_used_memory() as i64 }
        if self.cpu > -1.0 { self.cpu = system.get_global_processor_info().get_cpu_usage() }
        if self.system_uptime > -1 { self.system_uptime = system.get_uptime() as i64 }
        if self.cpu_average.0 > -1.0 {
            let avg_load = system.get_load_average();
            self.cpu_average = (avg_load.one, avg_load.five, avg_load.fifteen);
        }
    } 
}


#[cfg(test)]
mod tests {
    use super::super::super::parse_config::{Config, ConfigMode, LogType, LogCredentials};
    use super::IntervalMetrics;

    #[test]
    pub fn intervalmetrics_new_creates() {
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: true,
                cpu: true,
                cpu_average: true,
                system_uptime: true
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let metrics = IntervalMetrics::new(config);

        assert_ne!(metrics.ram, -1);
        assert_ne!(metrics.cpu, -1.0);
        assert_ne!(metrics.cpu_average, (-1.0, -1.0, -1.0));
        assert_ne!(metrics.system_uptime, -1);
    }


    #[test]
    #[should_panic = "The passed config mode does not have ConfigInterval as its mode."]
    fn intervalmetrics_new_wrong_mode() {
        let config = Config {
            mode: ConfigMode::ConfigWarn {
                cpu_limit: 20,
                ram_limit: 20
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        IntervalMetrics::new(config);
    }


    #[test]
    fn intervalmetrics_partly_enabled() {
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: true,
                cpu: true,
                cpu_average: false,
                system_uptime: false
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let metrics = IntervalMetrics::new(config);

        assert_ne!(metrics.ram, -1);
        assert_ne!(metrics.cpu, -1.0);
        assert_eq!(metrics.cpu_average, (-1.0, -1.0, -1.0));
        assert_eq!(metrics.system_uptime, -1);
    }

    // This test might not be the best one possible. Make sure to review it 
    // and perhaps change it with a better way of chaking the metrics updates.
    #[test]
    fn intervalmetrics_update_metrics_updates() {
        let config = Config {
            mode: ConfigMode::ConfigInterval {
                ram: false,
                cpu: false,
                cpu_average: false,
                system_uptime: true
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let mut metrics = IntervalMetrics::new(config);
        let old_uptime = metrics.system_uptime.clone();
        std::thread::sleep(std::time::Duration::new(1, 0));
        metrics.update_metrics();


        assert_ne!(old_uptime, metrics.system_uptime);
    }
}