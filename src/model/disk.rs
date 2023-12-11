/// Disk IO statistics.
#[derive(Debug, Clone)]
pub struct DiskIO {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}
