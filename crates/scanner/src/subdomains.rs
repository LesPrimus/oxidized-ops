use std::collections::BTreeSet;

use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CrtEntry {
    pub name_value: String,
}

pub fn entries(client: &Client, target: &str) -> reqwest::Result<BTreeSet<String>> {
    let target = target.trim().to_lowercase();
    let url = format!("https://crt.sh/?q=%25.{}&output=json", target);

    let subdomains: Vec<CrtEntry> = client.get(url).send()?.error_for_status()?.json()?;

    let mut entries: BTreeSet<String> = subdomains
        .iter()
        .flat_map(|e| e.name_value.split('\n').map(|d| d.trim().to_lowercase()))
        .filter(|e| e != &target)
        .filter(|e| !e.contains('*'))
        .collect();

    entries.insert(target);
    Ok(entries)
}
