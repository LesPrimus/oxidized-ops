use std::ops::RangeInclusive;

use clap::Parser;
use portscan::ports::parse_ports;


#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    pub domain: String,

    #[arg(short, long, value_parser = parse_ports, default_value = "1-1024")]
    pub ports: RangeInclusive<u16>,

    #[arg(short, long, default_value_t = 200)]
    pub councurrency: usize,
}
