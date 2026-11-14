use std::fmt::Display;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

impl Display for PortState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Open => "Open",
            Self::Closed => "Closed",
            Self::Filtered => "Filtered",
        };
        write!(f, "{result}")
    }
}

pub fn scan_port(add: SocketAddr, timeout: Duration) -> PortState {
    match TcpStream::connect_timeout(&add, timeout) {
        Ok(_) => PortState::Open,
        Err(e) if e.kind() == ErrorKind::ConnectionRefused => PortState::Closed,
        Err(_) => PortState::Filtered,
    }
}