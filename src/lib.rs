pub mod config;
pub mod result;
pub mod server;
pub mod system_info;

use futures::StreamExt;
use heim::{
    cpu, disk, host, memory,
    units::{information, ratio},
    Error,
};
use std::time::Duration;

async fn sys_info(timer: u64) -> std::result::Result<system_info::SystemInfo, Error> {
    let platform = host::platform().await?;
    let system = platform.system().to_string();
    let release = platform.release().to_string();
    let hostname = platform.hostname().to_string();
    let version = platform.version().to_string();
    let arch = platform.architecture().as_str().to_string();

    let memory = memory::memory().await?;
    let total = memory.total().get::<information::megabyte>();
    let free = memory.free().get::<information::megabyte>();
    let available = memory.available().get::<information::megabyte>();

    let partitions = disk::partitions_physical().await?;
    futures::pin_mut!(partitions);

    let mut disk: Vec<system_info::Disk> = Vec::new();
    while let Some(part) = partitions.next().await {
        let part = part?;
        let usage = disk::usage(part.mount_point().to_path_buf()).await?;

        let total = usage.total().get::<information::kibibyte>();
        let used = usage.used().get::<information::kibibyte>();
        let free = usage.free().get::<information::kibibyte>();
        let file_system = part.file_system().as_str().to_string();
        let mount_point = part.mount_point().to_string_lossy().to_string();

        disk.push(system_info::Disk {
            total,
            used,
            free,
            file_system,
            mount_point,
        });
    }
    let ip = local_ipaddress::get().unwrap_or("127.0.0.1".parse().unwrap());

    let platform = system_info::Platform {
        system,
        release,
        hostname,
        version,
        arch,
    };
    let net = vec![system_info::Net { ip }];
    let memory = system_info::Memory {
        total,
        free,
        available,
    };

    let measurement_1 = cpu::usage().await?;
    futures_timer::Delay::new(Duration::from_millis(timer)).await;
    let measurement_2 = cpu::usage().await?;

    let usage = (measurement_2 - measurement_1).get::<ratio::percent>();

    let count = cpu::logical_count().await?;
    let cpu = system_info::Cpu { count, usage };

    Ok(system_info::SystemInfo {
        platform,
        net,
        memory,
        disk,
        cpu,
    })
}
