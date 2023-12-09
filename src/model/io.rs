#[derive(Debug, Clone, Default)]
pub struct IOUsage {
    pub tot_read: u64,
    pub tot_write: u64,
    pub new_read: u64,
    pub new_write: u64,
}
