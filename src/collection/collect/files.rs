use std::fs::File;

/// File handles re-used for each container that read into the /proc VFS
pub struct ProcFileHandles {
    pub current_pids: Option<File>,
    pub max_pids: Option<File>,
    pub cpu_stat: Option<File>,
    pub cpuacct_stat: Option<File>,
    pub cpuacct_usage: Option<File>,
    pub cpuacct_usage_sys: Option<File>,
    pub cpuacct_usage_user: Option<File>,
    pub cpuacct_usage_percpu: Option<File>,
    pub memory_usage_in_bytes: Option<File>,
    pub memory_max_usage_in_bytes: Option<File>,
    pub memory_limit_in_bytes: Option<File>,
    pub memory_soft_limit_in_bytes: Option<File>,
    pub memory_failcnt: Option<File>,
    pub memory_stat: Option<File>,
    pub blkio_io_service_bytes: Option<File>,
    pub blkio_io_serviced: Option<File>,
    pub blkio_io_service_time: Option<File>,
    pub blkio_io_queued: Option<File>,
    pub blkio_io_wait_time: Option<File>,
    pub blkio_io_merged: Option<File>,
    pub blkio_time: Option<File>,
    pub blkio_sectors: Option<File>,
}

impl ProcFileHandles {
    /// Initializes all file handles to /proc files, utilizing them over the entire timeline of
    /// the container monitoring. If a handle fails to open, the struct field will be None
    pub fn new(id: &str) -> Self {
        ProcFileHandles {
            current_pids: open_proc_file(id, "pids", "pids.current"),
            max_pids: open_proc_file(id, "pids", "pids.max"),
            cpu_stat: open_proc_file(id, "cpu", "cpu.stat"),
            cpuacct_stat: open_proc_file(id, "cpuacct", "cpuacct.stat"),
            cpuacct_usage: open_proc_file(id, "cpuacct", "cpuacct.usage"),
            cpuacct_usage_sys: open_proc_file(id, "cpuacct", "cpuacct.usage_sys"),
            cpuacct_usage_user: open_proc_file(id, "cpuacct", "cpuacct.usage_user"),
            cpuacct_usage_percpu: open_proc_file(id, "cpuacct", "cpuacct.usage_percpu"),
            memory_usage_in_bytes: open_proc_file(id, "memory", "memory.usage_in_bytes"),
            memory_max_usage_in_bytes: open_proc_file(id, "memory", "memory.max_usage_in_bytes"),
            memory_limit_in_bytes: open_proc_file(id, "memory", "memory.limit_in_bytes"),
            memory_soft_limit_in_bytes: open_proc_file(id, "memory", "memory.soft_limit_in_bytes"),
            memory_failcnt: open_proc_file(id, "memory", "memory.failcnt"),
            memory_stat: open_proc_file(id, "memory", "memory.stat"),
            blkio_io_service_bytes: open_proc_file(id, "blkio", "blkio.io_service_bytes_recursive"),
            blkio_io_serviced: open_proc_file(id, "blkio", "blkio.io_serviced_recursive"),
            blkio_io_service_time: open_proc_file(id, "blkio", "blkio.io_service_time_recursive"),
            blkio_io_queued: open_proc_file(id, "blkio", "blkio.io_queued_recursive"),
            blkio_io_wait_time: open_proc_file(id, "blkio", "blkio.io_wait_time_recursive"),
            blkio_io_merged: open_proc_file(id, "blkio", "blkio.io_merged_recursive"),
            blkio_time: open_proc_file(id, "blkio", "blkio.time_recursive"),
            blkio_sectors: open_proc_file(id, "blkio", "blkio.sectors_recursive"),
        }
    }
}

/// Opens a stats file in /proc for the cgroup corresponding to the given container ID,
/// in the given subsystem
fn open_proc_file(id: &str, subsystem: &str, file: &str) -> Option<File> {
    File::open(format!(
        "/sys/fs/cgroup/{}/docker/{}/{}",
        subsystem, id, file
    ))
    .ok()
}
