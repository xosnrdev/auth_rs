#[derive(Clone)]
pub struct ServerConfig {
    address: String,
    port: u16,
    workers: usize,
}

impl ServerConfig {
    pub fn new(address: String, port: u16, workers: usize) -> Self {
        Self {
            address,
            port,
            workers,
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn workers(&self) -> usize {
        self.workers
    }
}
