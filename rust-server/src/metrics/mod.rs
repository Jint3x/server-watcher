use sysinfo::{DiskExt};


pub mod interval;
pub mod warn;


fn get_total_disk_space<T>(disks: &[T]) -> f64 where T: DiskExt {
    let mut total_space = 0;

    disks.iter().for_each(|disk| {
        total_space += disk.get_total_space();
    });

    (total_space / 1_000_000) as f64
}



fn get_used_disk_space<T>(disks: &[T]) -> i64 where T: DiskExt {
    let mut total_used = 0_i64;

    disks.iter().for_each(|disk| {
        total_used += ((disk.get_total_space() - disk.get_available_space())  / 1_000_000) as i64;
    });

    total_used
}


#[cfg(test)]
mod tests {
    use super::{get_total_disk_space, get_used_disk_space};
    use sysinfo::{DiskExt, DiskType};
    use std::ffi::OsStr;
    use std::path::Path;

    #[derive(Debug)]
    struct MockedDisk { available: u64, total: u64 }

    impl DiskExt for MockedDisk {
        fn get_type(&self) -> DiskType { DiskType::HDD }
        fn get_available_space(&self) -> u64 { self.available }
        fn get_file_system(&self) -> &[u8] { &[2, 3, 6, 4] }
        fn get_name(&self) -> &OsStr { OsStr::new("") }
        fn get_mount_point(&self) -> &Path { Path::new("") }
        fn get_total_space(&self) -> u64 { self.total }
        fn refresh(&mut self) -> bool { true }
    }


    #[test]
    fn get_used_disk_space_gets_used() {
        let disks = [
            MockedDisk {
                available: 1_000_000_000, // 1 GB 
                total: 5_000_000_000 // 5 GB
            },

            MockedDisk {
                available: 450_000_000, // 450 MB
                total: 2_000_000_000 // 2 GB
            }
        ];

        let used_space = get_used_disk_space(&disks);
        assert_eq!(used_space, ((5_000_000_000 - 1_000_000_000) + (2_000_000_000 - 450_000_000)) / 1000000);
    }


    #[test]
    fn get_total_disk_space_gets_all_space() {
        let disks = [
            MockedDisk {
                available: 1_000_000_000, // 1 GB 
                total: 5_000_000_000 // 5 GB
            },

            MockedDisk {
                available: 450_000_000, // 450 MB
                total: 2_000_000_000 // 2 GB
            }
        ];

        let total_space = get_total_disk_space(&disks);
        assert_eq!(total_space, 7000.0);
    }
}
