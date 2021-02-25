use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub platform: Platform,
    pub net: Vec<Net>,
    pub memory: Memory,
    pub disk: Vec<Disk>,
    pub cpu: Cpu,
}

#[derive(Debug, Serialize)]
pub struct Platform {
    pub system: String,
    pub release: String,
    pub hostname: String,
    pub version: String,
    pub arch: String,
}

#[derive(Debug, Serialize)]
pub struct Net {
    pub ip: String,
}

#[derive(Debug, Serialize)]
pub struct Memory {
    pub total: u64,
    pub available: u64,
    pub free: u64,
}

#[derive(Debug, Serialize)]
pub struct Disk {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub file_system: String,
    pub mount_point: String,
}

#[derive(Debug, Serialize)]
pub struct Cpu {
    pub count: u64,
    pub usage: f32,
}
