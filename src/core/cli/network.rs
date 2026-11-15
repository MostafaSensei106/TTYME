use std::net::SocketAddr;

use clap::Args;

use crate::core::constants::config::Constants;

use super::address::Host;

#[derive(Debug, Args)]
pub(super) struct NetworkArgs {
    /// Server address as IP or IP:PORT (IPv6 with a port must be bracketed)
    #[arg(long, value_name = "IP[:PORT]")]
    host: Option<Host>,

    /// Server port (overrides a port included in --host)
    #[arg(long, value_name = "PORT", value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,
}

impl NetworkArgs {
    pub(super) fn address(&self) -> SocketAddr {
        let defaults = Constants::default();
        let default_ip = defaults
            .host
            .parse()
            .expect("configured default host must be an IP");
        let mut address = SocketAddr::new(default_ip, defaults.port);

        if let Some(host) = self.host {
            address.set_ip(host.ip);
            if let Some(port) = host.port {
                address.set_port(port);
            }
        }
        if let Some(port) = self.port {
            address.set_port(port);
        }

        address
    }
}
