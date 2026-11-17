#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("DNS resolution failed: {0}")]
    Dns(#[from] hickory_resolver::net::NetError),
}
