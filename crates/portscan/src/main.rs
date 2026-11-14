use std::{error::Error, net::ToSocketAddrs};

use clap::Parser;

use crate::scanner::PortState;

mod cli;
mod ports;
mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let ports: Vec<u16> = args.ports.collect();

    let ip = (args.target.as_str(), 0)
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid Target")?
        .ip();

    let results = scanner::scan(ip, &ports, args.timeout, usize::from(args.threads));

    for (port, state) in results {
        if state == PortState::Open {
            println!("{port}/tcp {state}");
        }
    }
    Ok(())
}
