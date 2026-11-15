use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct Host {
    pub(super) ip: IpAddr,
    pub(super) port: Option<u16>,
}

impl FromStr for Host {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(address) = value.parse::<SocketAddr>() {
            if address.port() == 0 {
                return Err("port must be between 1 and 65535".into());
            }
            return Ok(Self {
                ip: address.ip(),
                port: Some(address.port()),
            });
        }

        value
            .parse::<IpAddr>()
            .map(|ip| Self { ip, port: None })
            .map_err(|_| format!("invalid host '{value}': expected an IP address or IP:PORT"))
    }
}
