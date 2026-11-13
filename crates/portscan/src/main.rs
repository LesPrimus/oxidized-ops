use clap::Parser;

mod cli;
mod ports;

fn main() {
    let args = cli::Args::parse();
    dbg!(args);
}
