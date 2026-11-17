use std::{collections::BTreeSet, net::IpAddr};

use futures::{StreamExt, stream};
use hickory_resolver::TokioResolver;
use reqwest::{self, Client};

use crate::{
    error::ScanError,
    models::{CrtEntry, SubDomain},
};

pub async fn crt_entries(client: &Client, target: &str) -> reqwest::Result<Vec<CrtEntry>> {
    let target = target.trim().to_lowercase();
    let url = format!("https://crt.sh/?q=%25.{}&output=json", target);

    let entries: Vec<CrtEntry> = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(entries)
}

pub async fn subdomains_names(client: &Client, target: &str) -> reqwest::Result<BTreeSet<String>> {
    let target = target.trim().to_lowercase();
    let entries = crt_entries(client, &target).await?;

    let mut unique_subdomains_names: BTreeSet<String> = entries
        .iter()
        .flat_map(|e| e.name_value.split("\n"))
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.contains("*"))
        .collect();

    unique_subdomains_names.insert(target);

    Ok(unique_subdomains_names)
}

pub async fn enumerate(
    client: &Client,
    target: &str,
    concurrency: usize,
) -> Result<Vec<SubDomain>, ScanError> {
    let names = subdomains_names(client, target).await?;
    let resolver = TokioResolver::builder_tokio()?.build()?;
    let ref_resolver = &resolver;

    let subdomains: Vec<SubDomain> = stream::iter(names)
        .map(|name| async move {
            let ips = resolve(&name, ref_resolver).await.ok()?;
            Some(SubDomain {
                name,
                ips: ips.into_iter().collect(),
            })
        })
        .buffer_unordered(concurrency)
        .filter_map(|sub| async move { sub })
        .collect()
        .await;
    Ok(subdomains)
}

pub async fn resolve(name: &str, resolver: &TokioResolver) -> Result<BTreeSet<IpAddr>, ScanError> {
    let ips: BTreeSet<IpAddr> = resolver.lookup_ip(name).await?.into_iter().collect();
    Ok(ips)
}
