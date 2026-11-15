use std::net::{IpAddr, SocketAddr};

use super::constants::config::Constants;

pub struct Options {
    pub username: Option<String>,
    pub address: SocketAddr,
    pub help: bool,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>, client: bool) -> Result<Self, String> {
        let defaults = Constants::default();
        let mut options = Self {
            username: None,
            address: SocketAddr::new(defaults.host.parse().unwrap(), defaults.port),
            help: false,
        };
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => options.help = true,
                "--host" => {
                    let value = args.next().ok_or("--host requires an IP or IP:PORT")?;
                    if let Ok(address) = value.parse::<SocketAddr>() {
                        options.address = address;
                    } else {
                        let ip = value.parse::<IpAddr>().map_err(|_| {
                            format!("Invalid host '{value}': expected an IP or IP:PORT")
                        })?;
                        options.address.set_ip(ip);
                    }
                }
                "--port" => {
                    let value = args.next().ok_or("--port requires a port number")?;
                    let port = value
                        .parse::<u16>()
                        .map_err(|_| format!("Invalid port '{value}': expected 1-65535"))?;
                    options.address.set_port(port);
                }
                _ if arg.starts_with('-') => return Err(format!("Unknown option: {arg}")),
                _ if client && options.username.is_none() && !arg.trim().is_empty() => {
                    options.username = Some(arg.trim().to_string());
                }
                _ => return Err(format!("Unexpected argument: {arg}")),
            }
        }
        if !options.help {
            if client && options.username.is_none() {
                return Err("Please provide a username as an argument".into());
            }
            if options.address.port() == 0 {
                return Err("Invalid port: expected 1-65535".into());
            }
        }
        Ok(options)
    }

    pub fn usage(client: bool) -> &'static str {
        if client {
            "Usage: app <username> [--host IP[:PORT]] [--port PORT]\nDefault: 127.0.0.1:8080\nExample: app Alice --host 192.168.1.10:9000\nUse [IPv6]:PORT for an IPv6 address with a port.\n-h, --help  Show this help"
        } else {
            "Usage: backend [--host IP[:PORT]] [--port PORT]\nDefault: 127.0.0.1:8080\nExample: backend --host 0.0.0.0:9000\nUse [IPv6]:PORT for an IPv6 address with a port.\n-h, --help  Show this help"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str], client: bool) -> Result<Options, String> {
        Options::parse(args.iter().map(|arg| arg.to_string()), client)
    }

    #[test]
    fn preserves_defaults() {
        let options = parse(&[" Alice "], true).unwrap();
        assert_eq!(options.username.as_deref(), Some("Alice"));
        assert_eq!(options.address.to_string(), "127.0.0.1:8080");
        assert_eq!(parse(&[], false).unwrap().address, options.address);
    }

    #[test]
    fn accepts_lan_endpoint_before_or_after_username() {
        for args in [
            vec!["Alice", "--host", "192.168.1.10:9000"],
            vec!["--host", "192.168.1.10:9000", "Alice"],
            vec!["--port", "9000", "Alice", "--host", "192.168.1.10"],
        ] {
            let options = parse(&args, true).unwrap();
            assert_eq!(options.username.as_deref(), Some("Alice"));
            assert_eq!(options.address.to_string(), "192.168.1.10:9000");
        }
        assert_eq!(
            parse(&["--host", "0.0.0.0", "--port", "9000"], false)
                .unwrap()
                .address
                .to_string(),
            "0.0.0.0:9000"
        );
    }

    #[test]
    fn accepts_ipv6() {
        for args in [
            vec!["Alice", "--host", "[::1]:9000"],
            vec!["Alice", "--host", "::1", "--port", "9000"],
        ] {
            assert_eq!(
                parse(&args, true).unwrap().address.to_string(),
                "[::1]:9000"
            );
        }
    }

    #[test]
    fn rejects_invalid_arguments() {
        for args in [
            vec![],
            vec!["Alice", "--host"],
            vec!["Alice", "--host", "invalid"],
            vec!["Alice", "--host", "192.168.1.10:65536"],
            vec!["Alice", "--host", "192.168.1.10:0"],
            vec!["Alice", "--port"],
            vec!["Alice", "--port", "abc"],
            vec!["Alice", "--port", "65536"],
            vec!["Alice", "--port", "0"],
            vec!["Alice", "--unknown"],
            vec!["Alice", "Bob"],
            vec![" "],
        ] {
            assert!(parse(&args, true).is_err(), "accepted {args:?}");
        }
        assert!(parse(&["Alice"], false).is_err());
    }

    #[test]
    fn help_does_not_require_username() {
        assert!(parse(&["--help"], true).unwrap().help);
        assert!(parse(&["-h"], false).unwrap().help);
    }
}
