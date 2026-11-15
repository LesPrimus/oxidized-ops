use std::collections::BTreeMap;
use std::net::IpAddr;
use std::{error::Error, time::Duration};

use clap::Parser;
use portscan::scanner::PortState;
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};

mod cli;
mod dns;
mod subdomains;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(32)
        .build_global()?;

    let client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(Duration::from_secs(30))
        .build()?;

    let entries = subdomains::entries(&client, &args.domain)?;

    let alive: Vec<dns::SubDomain> = entries
        .par_iter()
        .filter_map(|entry| dns::resolve(entry))
        .collect();

    let mut unique_ips: BTreeMap<IpAddr, Vec<String>> = BTreeMap::new();

    for dns::SubDomain { name, ips } in &alive {
        for ip in ips {
            unique_ips.entry(*ip).or_default().push(name.clone());
        }
    }

    let ports = [22, 80, 443, 8080];

    let scanned: Vec<(IpAddr, &Vec<String>, Vec<u16>)> = unique_ips
        .par_iter()
        .map(|(ip, names)| {
            let open: Vec<u16> =
                portscan::scanner::scan(*ip, &ports, Duration::from_millis(500), 10)
                    .into_iter()
                    .filter_map(|(port, state)| (state == PortState::Open).then_some(port))
                    .collect();

            (*ip, names, open)
        })
        .collect();

    for (ip, names, open) in &scanned {
        if !open.is_empty() {
            println!("{ip:<16} {:<40} {open:?}", names.join(", "));
        }
    }

    Ok(())
}
