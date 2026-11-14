use std::{error::Error, net::{SocketAddr, ToSocketAddrs}};

use clap::Parser;

mod cli;
mod ports;
mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let ip = (args.target.as_str(), 0)
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid Target")?
        .ip();

    for port in args.ports.clone() {
        let addr = SocketAddr::new(ip, port);
        let is_open = scanner::scan_port(addr, args.timeout);
        println!("{addr} -> {is_open}");
    }
    Ok(())
}
