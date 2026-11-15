use std::net::SocketAddr;

use clap::Parser;

use super::network::NetworkArgs;

/// TTYME terminal chat client options.
#[derive(Debug, Parser)]
#[command(
    name = "app",
    version,
    about = "Connect to a TTYME chat server",
    long_about = None,
    after_help = "Default server: 127.0.0.1:8080\nExample: app Mostafa --host 192.168.1.10:9000"
)]
pub struct ClientOptions {
    /// Name displayed to other chat participants
    #[arg(value_name = "USERNAME", value_parser = parse_username)]
    pub username: String,

    #[command(flatten)]
    network: NetworkArgs,
}

impl ClientOptions {
    pub fn address(&self) -> SocketAddr {
        self.network.address()
    }
}

fn parse_username(value: &str) -> Result<String, String> {
    let username = value.trim();
    if username.is_empty() {
        return Err("username cannot be empty".into());
    }
    Ok(username.to_owned())
}
