pub enum Config {
    ServerConfig(ServerConfig),
    ClientConfig(ClientConfig),
}

pub struct ServerConfig {
    pub client_port: String,
    pub customer_port: String,
}

pub struct ClientConfig {
    pub service_port: String,
    pub server_address: String,
}
