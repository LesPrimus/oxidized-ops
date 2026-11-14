use std::{error::Error, net::{SocketAddr, ToSocketAddrs}};
use std::sync::mpsc::channel;
use std::thread;

use clap::Parser;

mod cli;
mod ports;
mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let ports: Vec<u16> = args.ports.collect();
    let chunk_size = ports.len().div_ceil(usize::from(args.threads));

    let ip = (args.target.as_str(), 0)
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid Target")?
        .ip();

    let timeout = args.timeout;
    
    let (sender, receiver) = channel();
    
    for chunk in ports.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let cloned_sender = sender.clone();
        thread::spawn(move || {
            for port in chunk {
                let addr = SocketAddr::new(ip, port);
                let result = scanner::scan_port(addr, timeout);
                cloned_sender.send((port, result)).unwrap();
            } 
        });
    }
    drop(sender);

    for (port, state) in receiver {
        if state == scanner::PortState::Open {
            println!("{port}/tcp {state}");
        }
    }
    
    Ok(())
}
