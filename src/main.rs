use cds::{Result, Server};
use codecrafters_dns_server as cds;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    Server::bind("127.0.0.1:2053")?
        .handler(|buf| {
            println!("Received {} bytes", buf.len());
            vec![]
        })
        .run()
}
