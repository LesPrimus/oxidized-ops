use std::{collections::BTreeSet, net::IpAddr};

use futures::{StreamExt, stream};
use hickory_resolver::TokioResolver;

use crate::{error::ScanError, models::SubDomain, modules::SubDomainModule};

pub async fn discover(
    modules: &[Box<dyn SubDomainModule>],
    target: &str,
    concurrency: usize,
) -> Result<Vec<SubDomain>, ScanError> {
    let names = names(modules, target).await;
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

pub async fn names(modules: &[Box<dyn SubDomainModule>], target: &str) -> BTreeSet<String> {
    let target = target.trim().to_lowercase();
    let mut names = BTreeSet::new();

    for module in modules {
        match module.enumerate(&target).await {
            Ok(found) => names.extend(
                found
                    .iter()
                    .map(|name| name.trim().to_lowercase())
                    .filter(|name| !name.contains('*')),
            ),
            Err(e) => eprintln!("[{}] failed: {e}", module.name()),
        }
    }

    names.insert(target);
    names
}
