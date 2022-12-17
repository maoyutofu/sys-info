use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: Platform,
    pub net: Vec<Net>,
    pub memory: Memory,
    pub disk: Vec<Disk>,
    pub cpu: Cpu,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub system: String,
    pub release: String,
    pub hostname: String,
    pub version: String,
    pub arch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Net {
    pub ip: String,
    pub name: String,
    pub ip_v6: String,
    pub mac: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
}

impl Eq for Net {}

impl PartialEq<Self> for Net {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd<Self> for Net {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Net {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_uppercase().cmp(&other.name.to_uppercase())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub total: u64,
    pub available: u64,
    pub free: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub file_system: String,
    pub mount_point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    pub count: u64,
    pub usage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineId {
    pub id: String,
}
