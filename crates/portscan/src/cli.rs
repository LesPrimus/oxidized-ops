use std::ops::RangeInclusive;
use std::time::Duration;

use clap::Parser;

use portscan::ports::parse_ports;

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

    #[arg(short = 'T', long, default_value_t = 100, value_parser = clap::value_parser!(u16).range(1..=1000))]
    pub threads: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert();
    }
}
