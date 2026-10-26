pub struct Constants {
    pub host: &'static str,
    pub port: u16,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            host: "127.0.0.1",
            port: 8080,
        }
    }
}
