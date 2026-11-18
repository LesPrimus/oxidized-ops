use std::ops::RangeInclusive;

use clap::{Args, Parser, Subcommand};
use portscan::ports::parse_ports;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List all available modules
    Modules,
    /// Discover subdomains of a domain and scan their ports.
    Scan(ScanArgs),
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Target domain, e.g. example.com
    pub domain: String,

    #[arg(short, long, value_parser = parse_ports, default_value = "1-1024")]
    pub ports: RangeInclusive<u16>,

    #[arg(short, long, default_value_t = 200,
          value_parser = clap::builder::RangedU64ValueParser::<usize>::new().range(1..=10_000))]
    pub concurrency: usize,
}
