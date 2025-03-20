use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlockConnectError {
    ConnectionError(capnp::Error),
    IoError(std::io::Error),
    InvalidBlockData,
    NodeError(String),
}

impl fmt::Display for BlockConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockConnectError::ConnectionError(e) => write!(f, "IPC connection error: {}", e),
            BlockConnectError::IoError(e) => write!(f, "IO error: {}", e),
            BlockConnectError::InvalidBlockData => write!(f, "Invalid block data"),
            BlockConnectError::NodeError(s) => write!(f, "Node error: {}", s),
        }
    }
}

impl Error for BlockConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BlockConnectError::ConnectionError(e) => Some(e),
            BlockConnectError::IoError(e) => Some(e),
            BlockConnectError::InvalidBlockData => None,
            BlockConnectError::NodeError(_) => None,
        }
    }
}

impl From<capnp::Error> for BlockConnectError {
    fn from(error: capnp::Error) -> Self {
        BlockConnectError::ConnectionError(error)
    }
}

impl From<std::io::Error> for BlockConnectError {
    fn from(error: std::io::Error) -> Self {
        BlockConnectError::IoError(error)
    }
}
