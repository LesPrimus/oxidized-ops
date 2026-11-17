use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug)]
pub struct SubDomain {
    pub name: String,
    pub ips: Vec<IpAddr>,
}

#[derive(Debug, Deserialize)]
pub struct CrtEntry {
    pub name_value: String,
}
