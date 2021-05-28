use sysinfo::{ProcessorExt, SystemExt, System};
use super::super::parse_config::{Config, ConfigMode};
use super::{get_total_disk_space, get_used_disk_space};

#[derive(Debug, PartialEq)]
pub enum Warn {
    HighRAM(f32),
    HighCPU(f32),
    HighDisk(f32),
    HighSwap(f32),
}


pub enum MetricType {
    RAM,
    CPU,
    Disk,
    Swap,
}


pub struct WarnMetrics {
    // RAM Limit (%)
    pub ram: u32,

    // CPU Limit (%)
    pub cpu: u32,

    // Disk usage (%) from all disks combined
    pub disk: u32,

    // Swap usage (%)
    pub swap: u32,

    // List of warnings for the different metrics if they go above limit
    pub warnings: Vec<Warn>
}


impl WarnMetrics {
    pub fn new(config: &Config) -> WarnMetrics {
        if let ConfigMode::ConfigWarn { ram_limit, cpu_limit, disk_limit, swap_limit } = config.mode {
            WarnMetrics {
                ram: ram_limit,
                cpu: cpu_limit,
                disk: disk_limit,
                swap: swap_limit,
                warnings: vec![],
            }
        } else {
            panic!("The passed config mode does not have ConfigWarn as its mode.")
        }
    }


    pub fn update_warns(&mut self, system: &mut System) {
        self.warnings.clear();
        system.refresh_all();

        // Check system RAM 
        if self.ram > 0 {
            let limit = above_limit(
    self.ram as f64,
    system.get_total_memory() as f64,
     system.get_used_memory() as f64, 
                MetricType::RAM
            );

            if let Ok(warn) = limit { self.warnings.push(warn) }
        } 

        // Check system CPU
        if self.cpu > 0 {
            let limit = above_limit(
    self.cpu as f64,
    100 as f64,
     system.get_global_processor_info().get_cpu_usage() as f64,
                MetricType::CPU
            );

            if let Ok(warn) = limit { self.warnings.push(warn) }
        }

        // Check system space
        if self.disk > 0 {
            let limit = above_limit(
    self.disk as f64, 
    get_total_disk_space(system.get_disks()), 
     get_used_disk_space(system.get_disks()) as f64, 
                MetricType::Disk
            );

            if let Ok(warn) = limit { self.warnings.push(warn) }
        }

        if self.swap > 0 {
            let limit = above_limit(
    self.swap as f64, 
    system.get_total_swap() as f64, 
     system.get_used_swap() as f64, 
                MetricType::Swap
            );

            if let Ok(warn) = limit { self.warnings.push(warn) }
        }
    }
}


// Check to see if the current metric usage is above the passed percentage.
// If it is, return the % of the metric that is used to the warn vector.
fn above_limit(metric_limit: f64, total_metric: f64, used_metric: f64, metric_type: MetricType) -> Result<Warn, bool> {
    let used_percentage = (used_metric / total_metric) * 100.0;
    
    if used_percentage > metric_limit {
        match metric_type {
            MetricType::RAM => Ok(Warn::HighRAM(used_percentage as f32)),
            MetricType::CPU => Ok(Warn::HighCPU(used_percentage as f32)),
            MetricType::Disk => Ok(Warn::HighDisk(used_percentage as f32)),
            MetricType::Swap => Ok(Warn::HighSwap(used_percentage as f32)),
        }
    } else {
        Err(false)
    }
}


#[cfg(test)]
mod tests {
    use super::super::super::parse_config::{Config, ConfigMode, LogType, LogCredentials};
    use super::{WarnMetrics, above_limit, MetricType, Warn};


    #[test]
    pub fn metricwarns_creates() {
        let config = Config {
            mode: ConfigMode::ConfigWarn {
                ram_limit: 40,
                cpu_limit: 45,
                disk_limit: 50,
                swap_limit: 34,
            },
            interval: 10,
            log_type: LogType::Discord,
            log_credentials: LogCredentials::DiscordLog {
                key: "secret_key".to_string(),
                channel: 123456789
            }
        };

        let metric_warns = WarnMetrics::new(&config);

        assert_eq!(metric_warns.cpu, 45);
        assert_eq!(metric_warns.ram, 40);
        assert_eq!(metric_warns.disk, 50);
        assert_eq!(metric_warns.swap, 34);
        assert_eq!(metric_warns.warnings.len(), 0);
    }


    #[test]
    #[should_panic = "The passed config mode does not have ConfigWarn as its mode."]
    pub fn metricwarns_wrong_mode() {
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

        WarnMetrics::new(&config);
    }


    #[test]
    pub fn above_limit_below_limit() {
        let limit = above_limit(
20.0,
100.0,
 15.0,
            MetricType::RAM
        );

        if let Err(above_limit) = limit {
            assert_eq!(above_limit, false)
        } else {
            panic!("It should have been an error")
        }
    }


    #[test]
    pub fn above_limit_above_limit() {
        let limit = above_limit(
20.0,
100.0,
 50.0,
            MetricType::RAM
        );

        if let Ok(above_limit) = limit {
            assert_eq!(above_limit, Warn::HighRAM(50.0))
        } else {
            panic!("It should have been an error")
        }
    }
}



