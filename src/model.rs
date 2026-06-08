use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListenerPort {
    pub port: u16,
    pub pid: u32,
    pub process_name: String,
    pub scope: PortScope,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortScope {
    Loopback,
    AllInterfaces,
    Public,
}

#[derive(Debug, Error)]
pub enum ExitPortError {
    #[error("failed to read listening sockets: {0}")]
    SocketScan(#[from] netstat2::error::Error),
    #[error("port has no owning process id")]
    MissingPid,
    #[error("process {0} is no longer running")]
    ProcessGone(u32),
    #[error("failed to stop process {pid} ({name})")]
    KillFailed { pid: u32, name: String },
}

pub type Result<T> = std::result::Result<T, ExitPortError>;
