use std::net::SocketAddr;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub endpoint: SocketAddr,
    pub buffer_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: "127.0.0.1:7878".parse().unwrap(),
            buffer_size: 2048,
        }
    }
}
