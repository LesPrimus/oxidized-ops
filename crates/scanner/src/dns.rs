use std::net::{IpAddr, ToSocketAddrs};

#[derive(Debug)]
pub struct SubDomain {
    pub name: String,
    pub ips: Vec<IpAddr>,
}

pub fn resolve(name: &str) -> Option<SubDomain> {
    let mut ips: Vec<IpAddr> = (name, 0)
        .to_socket_addrs()
        .ok()?
        .map(|addr| addr.ip())
        .collect();

    if ips.is_empty() {
        return None;
    }

    ips.sort_unstable();
    ips.dedup();
    Some(SubDomain {
        name: name.into(),
        ips,
    })
}
