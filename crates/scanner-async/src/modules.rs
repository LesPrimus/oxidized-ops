pub mod subdomains;

use std::collections::BTreeSet;

use async_trait::async_trait;
use reqwest::Client;

use crate::error::ScanError;

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubDomainModule: Module + Send + Sync {
    async fn enumerate(&self, domain: &str) -> Result<BTreeSet<String>, ScanError>;
}

pub fn subdomain_modules(client: &Client) -> Vec<Box<dyn SubDomainModule>> {
    vec![
        Box::new(subdomains::CrtSh::new(client.clone())),
        Box::new(subdomains::WebArchive::new(client.clone())),
    ]
}
