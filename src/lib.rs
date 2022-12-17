use std::{collections::HashMap, time::Duration};

use futures::StreamExt;
#[cfg(target_os = "linux")]
use heim::net::os::linux::IoCountersExt;
#[cfg(target_os = "windows")]
use heim::net::os::windows::IoCountersExt;
use heim::{
    cpu, disk, host, memory, net,
    units::{information, ratio},
    Error,
};

use machineid_rs::HWIDComponent;
use machineid_rs::{Encryption, IdBuilder};

pub mod config;
pub mod result;
pub mod server;
pub mod system_info;

fn str_md5(str: &[u8]) -> String {
    // let str = format!("{:x}", md5::compute(str));
    // str[8..24].to_string()
    format!("{:x}", md5::compute(str))
}

/// 获取deviceId
/// 采用机器唯一标识符、计算机的物理内核数、处理器序列号、key 进行 SHA256 计算得到deviceId
async fn get_device_id() -> system_info::MachineId {
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores)
        .add_component(HWIDComponent::CPUID);
    let device_id = builder.build("").unwrap_or(String::from(""));
    
    system_info::MachineId {
        id: str_md5(device_id.as_bytes())
    }
}

async fn sys_platform() -> std::result::Result<system_info::Platform, Error> {
    let platform = host::platform().await?;
    let system = platform.system().to_string();
    let release = platform.release().to_string();
    let hostname = platform.hostname().to_string();
    let version = platform.version().to_string();
    let arch = platform.architecture().as_str().to_string();

    let platform = system_info::Platform {
        system,
        release,
        hostname,
        version,
        arch,
    };

    Ok(platform)
}

async fn sys_memory() -> std::result::Result<system_info::Memory, Error> {
    let memory = memory::memory().await?;
    let total = memory.total().get::<information::megabyte>();
    let free = memory.free().get::<information::megabyte>();
    let available = memory.available().get::<information::megabyte>();

    let memory = system_info::Memory {
        total,
        free,
        available,
    };
    Ok(memory)
}


async fn sys_disk() -> std::result::Result<Vec<system_info::Disk>, Error> {
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

    Ok(disk)
}

async fn sys_net() -> std::result::Result<Vec<system_info::Net>, Error> {
    let mut map: HashMap<String, HashMap<&str, String>> = HashMap::new();

    let nic = net::nic().await?;
    futures::pin_mut!(nic);
    while let Some(iface) = nic.next().await {
        let iface = iface?;

        let name = iface.name().to_string();

        let new_tmp: HashMap<&str, String> = HashMap::new();

        let mut tmp: HashMap<&str, String> = match map.get(&name) {
            Some(x) => x.to_owned(),
            None => new_tmp,
        };

        let addr = iface.address();

        match addr {
            net::Address::Inet(addr) => {
                tmp.insert("ip", addr.ip().to_string());
            }
            net::Address::Inet6(addr) => {
                tmp.insert("ip_v6", addr.ip().to_string());
            }
            net::Address::Link(addr) => {
                tmp.insert("mac", addr.to_string());
            }
            _ => {}
        }

        map.insert(name, tmp);
    }

    let counters = net::io_counters().await?;
    futures::pin_mut!(counters);
    while let Some(counter) = counters.next().await {
        if counter.is_err() {
            continue;
        }

        let counter = counter.unwrap();

        let inter = counter.interface();

        let tmp = map.get_mut(inter);
        if tmp.is_none() {
            continue;
        }
        let tmp = tmp.unwrap();

        let bytes_sent = counter.bytes_sent();
        tmp.insert("bytes_sent", bytes_sent.value.to_string());

        let bytes_recv = counter.bytes_recv();
        tmp.insert("bytes_recv", bytes_recv.value.to_string());

        let packets_sent = counter.packets_sent();
        tmp.insert("packets_sent", packets_sent.to_string());

        let packets_recv = counter.packets_recv();
        tmp.insert("packets_recv", packets_recv.to_string());

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let _ = counter.drop_sent();
    }

    let mut net = vec![];

    for (key, value) in map.iter() {
        let name = key.into();
        let ip = value.get("ip").unwrap_or(&String::new()).into();
        let ip_v6 = value.get("ip_v6").unwrap_or(&String::new()).into();
        let mac = value.get("mac").unwrap_or(&String::new()).into();
        let bytes_sent_str: String = value.get("bytes_sent").unwrap_or(&"0".to_string()).into();
        let bytes_recv_str: String = value.get("bytes_recv").unwrap_or(&"0".to_string()).into();
        let packets_sent_str: String = value.get("packets_sent").unwrap_or(&"0".to_string()).into();
        let packets_recv_str: String = value.get("packets_recv").unwrap_or(&"0".to_string()).into();

        let bytes_sent = bytes_sent_str.parse::<u64>()?;
        let bytes_recv = bytes_recv_str.parse::<u64>()?;
        let packets_sent = packets_sent_str.parse::<u64>()?;
        let packets_recv = packets_recv_str.parse::<u64>()?;

        net.push(system_info::Net {
            name,
            ip,
            ip_v6,
            mac,
            bytes_sent,
            bytes_recv,
            packets_sent,
            packets_recv,
        });
    }

    net.sort_unstable();

    Ok(net)
}


async fn sys_cpu(timer: u64) -> std::result::Result<system_info::Cpu, Error> {
    let measurement_1 = cpu::usage().await?;
    futures_timer::Delay::new(Duration::from_millis(timer)).await;
    let measurement_2 = cpu::usage().await?;

    let usage = (measurement_2 - measurement_1).get::<ratio::percent>();

    let count = cpu::logical_count().await?;
    let cpu = system_info::Cpu { count, usage };

    Ok(cpu)
}

async fn sys_info(timer: u64) -> std::result::Result<system_info::SystemInfo, Error> {
    let platform = sys_platform().await?;
    let net = sys_net().await?;
    let memory = sys_memory().await?;
    let disk = sys_disk().await?;
    let cpu = sys_cpu(timer).await?;

    Ok(system_info::SystemInfo {
        platform,
        net,
        memory,
        disk,
        cpu,
    })
}
