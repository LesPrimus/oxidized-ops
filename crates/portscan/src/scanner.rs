use std::fmt::Display;
use std::io::ErrorKind;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::thread;
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

pub fn scan_port(addr: SocketAddr, timeout: Duration) -> PortState {
    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => PortState::Open,
        Err(e) if e.kind() == ErrorKind::ConnectionRefused => PortState::Closed,
        Err(_) => PortState::Filtered,
    }
}

/// Scans `ports` on `ip` using up to `threads` threads.
/// Results are returned in the same order as `ports`.
pub fn scan(ip: IpAddr, ports: &[u16], timeout: Duration, threads: usize) -> Vec<(u16, PortState)> {
    let chunk_size = ports.len().div_ceil(threads).max(1);

    thread::scope(|s| {
        let handles: Vec<_> = ports
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(move || {
                    chunk
                        .iter()
                        .map(|&port| (port, scan_port(SocketAddr::new(ip, port), timeout)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("scan thread panicked"))
            .collect()
    })
}
