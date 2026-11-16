use std::{collections::BTreeSet, error::Error, net::IpAddr};

use hickory_resolver::TokioResolver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resolver = TokioResolver::builder_tokio()?.build()?;

    let ips: BTreeSet<IpAddr> = resolver
        .lookup_ip("www.example.com")
        .await?
        .into_iter()
        .collect();

    dbg!(ips);
    Ok(())
}
