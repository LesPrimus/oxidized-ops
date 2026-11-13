use std::ops::RangeInclusive;
use std::time::Duration;

use clap::Parser;

use crate::ports::parse_ports;

fn parse_millis(s: &str) -> Result<Duration, std::num::ParseIntError> {
    s.parse().map(Duration::from_millis)
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub target: String,

    #[arg(short, long, value_parser = parse_ports, default_value = "1-1024")]
    pub ports: RangeInclusive<u16>,

    #[arg(short, long, value_parser = parse_millis, default_value = "200")]
    pub timeout: Duration,
}
