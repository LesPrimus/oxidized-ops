use std::{
    collections::{BTreeMap, BTreeSet},
    io::ErrorKind,
    net::{IpAddr, SocketAddr},
    ops::RangeInclusive,
    time::Duration,
};

use futures::{StreamExt, stream};
use tokio::{net::TcpStream, time::timeout};

use crate::models::PortState;

pub async fn scan_port(addr: SocketAddr, t: Duration) -> PortState {
    match timeout(t, TcpStream::connect(addr)).await {
        Ok(Ok(_)) => PortState::Open,
        Ok(Err(e)) if e.kind() == ErrorKind::ConnectionRefused => PortState::Closed,
        _ => PortState::Filtered,
    }
}

pub async fn scan(
    ips: impl IntoIterator<Item = IpAddr>,
    ports: RangeInclusive<u16>,
    t: Duration,
    concurrency: usize,
) -> BTreeMap<IpAddr, BTreeSet<u16>> {
    let targets = ips
        .into_iter()
        .flat_map(|ip| ports.clone().map(move |p| SocketAddr::new(ip, p)));

    let open: Vec<SocketAddr> = stream::iter(targets)
        .map(|addr| async move { (addr, scan_port(addr, t).await) })
        .buffer_unordered(concurrency)
        .filter_map(|(addr, state)| async move { (state == PortState::Open).then_some(addr) })
        .collect()
        .await;

    let mut by_ip: BTreeMap<IpAddr, BTreeSet<u16>> = BTreeMap::new();

    for addr in open {
        by_ip.entry(addr.ip()).or_default().insert(addr.port());
    }
    by_ip
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn scan_port_open_then_closed() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        assert_eq!(
            scan_port(addr, Duration::from_millis(500)).await,
            PortState::Open
        );

        drop(listener);
        assert_eq!(
            scan_port(addr, Duration::from_millis(500)).await,
            PortState::Closed
        );
    }

    #[tokio::test]
    async fn scan_finds_listening_port() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let open_port = listener.local_addr().unwrap().port();
        let localhost: IpAddr = [127, 0, 0, 1].into();

        let ports = open_port.saturating_sub(2)..=open_port.saturating_add(2);
        let result = scan([localhost], ports, Duration::from_millis(500), 10).await;

        assert_eq!(result.len(), 1);
        assert!(result[&localhost].contains(&open_port));
    }
}
