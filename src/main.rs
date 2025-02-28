use cds::{Args, Result, Server};
use clap::Parser;
use codecrafters_dns_server as cds;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    Server::bind("127.0.0.1:2053")?
        .resolver(args.resolver)?
        .run()
}
