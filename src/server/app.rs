use std::collections::VecDeque;
use crate::server::builder::ServerBuilder;
use crate::server::web_server::Server;

pub fn listen(address: &str) -> ServerBuilder {
  ServerBuilder {
    address: address.to_string(),
    middleware: VecDeque::new(),
  }
}