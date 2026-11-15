use std::net::SocketAddr;

use clap::Parser;

use super::network::NetworkArgs;

/// TTYME chat server options.
#[derive(Debug, Parser)]
#[command(
    name = "backend",
    version,
    about = "Run the TTYME chat server",
    long_about = None,
    after_help = "Default server: 127.0.0.1:8080\nExample: backend --host 0.0.0.0:9000"
)]
pub struct BackendOptions {
    #[command(flatten)]
    network: NetworkArgs,
}

impl BackendOptions {
    pub fn address(&self) -> SocketAddr {
        self.network.address()
    }
}
