use std::time::Instant;
use std::{error::Error, time::Duration};

use clap::Parser;
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

    let start = Instant::now();

    let alive: Vec<dns::SubDomain> = entries
        .par_iter()
        .filter_map(|entry| dns::resolve(entry))
        .collect();

    println!("{:?}", start.elapsed());

    for sub in &alive {
        println!("{:<35} {:?}", sub.name, sub.ips);
    }

    Ok(())
}
