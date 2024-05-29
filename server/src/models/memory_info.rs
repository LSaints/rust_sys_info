use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct MemoryInfo {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,

    total_swap: u64,
    used_swap: u64,
    free_swap: u64,
}

impl MemoryInfo {
    pub fn new() -> Self {
        let sys: System = System::new_all();
        Self {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            free_memory: sys.free_memory(),

            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            free_swap: sys.free_swap(),
        }
    }
}