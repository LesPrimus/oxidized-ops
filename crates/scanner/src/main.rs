use std::{error::Error, time::Duration};

use clap::Parser;
use reqwest::{blocking::Client, redirect};

mod cli;
mod subdomains;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    let target = args.domain;

    let client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(Duration::from_secs(30))
        .build()?;

    let entries = subdomains::entries(&client, &target)?;
    dbg!(entries);
    Ok(())
}
