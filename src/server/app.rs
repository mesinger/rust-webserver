use crate::server::web_server::Server;

pub fn listen(address: &str) -> Server {
  Server {
    address: address.to_string(),
    middleware: vec![],
  }
}