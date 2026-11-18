use std::collections::BTreeSet;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    error::ScanError,
    models::CrtEntry,
    modules::{Module, SubDomainModule},
};

pub struct CrtSh {
    client: Client,
}

impl CrtSh {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Module for CrtSh {
    fn name(&self) -> String {
        "crtsh".to_string()
    }

    fn description(&self) -> String {
        "Find subdomains in certificate transparency logs (crt.sh)".to_string()
    }
}

#[async_trait]
impl SubDomainModule for CrtSh {
    async fn enumerate(&self, domain: &str) -> Result<BTreeSet<String>, ScanError> {
        let url = format!("https://crt.sh/?q=%25.{domain}&output=json");
        let entries: Vec<CrtEntry> = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(entries
            .iter()
            .flat_map(|e| e.name_value.split('\n'))
            .map(str::to_owned)
            .collect())
    }
}
