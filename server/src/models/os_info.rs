use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct OsInfo {
    name: String,
    kernel_version: String,
    system_version: String,
    host_name: String,
}

impl OsInfo {
    pub fn new() -> Self {
        Self { 
            name: String::from(System::name().unwrap()),
            kernel_version: String::from(System::kernel_version().unwrap()),
            system_version: String::from(System::os_version().unwrap()),
            host_name: String::from(System::host_name().unwrap()) 
        }
    }
}