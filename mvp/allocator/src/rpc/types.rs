use reqwest::blocking::Client;

//Connection struct to perform JSON-RPC requests
pub struct Connection {
    pub client: Client,
    pub rpc_url: String,
}
impl Connection {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: Client::new(),
            rpc_url: rpc_url.to_string(),
        }
    }
}
