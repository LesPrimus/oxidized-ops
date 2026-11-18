use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    net::IpAddr,
    time::Duration,
};

use clap::Parser;
use reqwest::Client;

use crate::{
    cli::{Command, ScanArgs},
    error::ScanError,
    models::SubDomain,
};

mod cli;
mod error;
mod models;
mod modules;
mod port_scan;
mod subdomains;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    let http_timeout = Duration::from_secs(30);
    let client = Client::builder().timeout(http_timeout).build()?;

    match cli.command {
        Command::Modules => list_modules(&client),
        Command::Scan(args) => scan(&client, args).await?,
    }

    Ok(())
}

fn list_modules(client: &Client) {
    println!("Subdomain modules");
    for module in modules::subdomain_modules(client) {
        println!("  {:<12} {}", module.name(), module.description());
    }
}

async fn scan(client: &Client, args: ScanArgs) -> Result<(), ScanError> {
    let modules = modules::subdomain_modules(client);

    let sub_domains = subdomains::discover(&modules, &args.domain, args.concurrency).await?;

    let mut names_by_ip: BTreeMap<IpAddr, BTreeSet<String>> = BTreeMap::new();
    for SubDomain { name, ips } in sub_domains {
        for ip in ips {
            names_by_ip.entry(ip).or_default().insert(name.clone());
        }
    }

    let open = port_scan::scan(
        names_by_ip.keys().copied(),
        args.ports,
        Duration::from_millis(500),
        args.concurrency,
    )
    .await;

    for (ip, ports) in &open {
        let names: Vec<&str> = names_by_ip[ip].iter().map(String::as_str).collect();
        println!("{ip:<40} {:<40} {ports:?}", names.join(", "));
    }
    Ok(())
}
