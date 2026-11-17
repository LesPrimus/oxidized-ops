use std::{collections::BTreeSet, error::Error, net::IpAddr, time::Duration};

use clap::Parser;
use hickory_resolver::TokioResolver;
use reqwest::Client;

mod cli;
mod models;
mod subdomains;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    
    let http_timeout = Duration::from_secs(30);
    let client = Client::builder().timeout(http_timeout).build()?;
    
    
    let s = subdomains::subdomains(&client, &args.domain).await?;

    dbg!(s);
    Ok(())
}
