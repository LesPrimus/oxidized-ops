use std::{error::Error, time::Duration};

use clap::Parser;
use reqwest::Client;

use crate::models::SubDomain;

mod cli;
mod error;
mod models;
mod subdomains;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    let http_timeout = Duration::from_secs(30);
    let client = Client::builder().timeout(http_timeout).build()?;

    let sub_domains = subdomains::enumerate(&client, &args.domain, args.concurrency).await?;

    for SubDomain { name, ips } in sub_domains.iter() {
        println!("{name}: -> {ips:?}");
    }
    Ok(())
}
