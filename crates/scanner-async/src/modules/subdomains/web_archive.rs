use std::{collections::BTreeSet, time::Duration};

use async_trait::async_trait;
use reqwest::{Client, Url};

use crate::{
    error::ScanError,
    modules::{Module, SubDomainModule},
};

pub struct WebArchive {
    client: Client,
}

impl WebArchive {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Module for WebArchive {
    fn name(&self) -> String {
        "web_archive".to_string()
    }

    fn description(&self) -> String {
        "Find subdomains in web archive".to_string()
    }
}

#[async_trait]
impl SubDomainModule for WebArchive {
    async fn enumerate(&self, domain: &str) -> Result<BTreeSet<String>, ScanError> {
        let url = format!(
            "https://web.archive.org/cdx/search/cdx?matchType=domain&fl=original&output=json&collapse=urlkey&url={domain}"
        );

        let rows: Vec<Vec<String>> = self
            .client
            .get(url)
            .timeout(Duration::from_secs(120))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(rows
            .into_iter()
            .skip(1) // header row: ["original"]
            .flatten()
            .filter_map(|raw| {
                let url = Url::parse(&raw).ok()?;
                Some(url.host_str()?.trim_end_matches('.').to_owned())
            })
            .collect())
    }
}
