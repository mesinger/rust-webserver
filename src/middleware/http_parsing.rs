use std::collections::HashMap;
use core::str::Split;
use crate::core::context::{ServerContext, ServerHttpRequest};
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct HttpParsingMiddleware {

}

impl Middleware for HttpParsingMiddleware {
  fn action(&self, context: &mut ServerContext) {
    let req = self.parse_request(&context.raw_message);
    context.request = req;
  }
}

impl HttpParsingMiddleware {
  fn parse_request(&self, content: &str) -> ServerHttpRequest {
    let mut lines = content.split('\n');
    let (method, path) = self.parse_first_line(lines.next().unwrap());

    let others = self.parse_others(&mut lines);
    let host = others.get("Host").unwrap();
    let user_agent = others.get("User-Agent").unwrap();

    ServerHttpRequest {
      method,
      path,
      host: host.clone(),
      user_agent: user_agent.clone(),
    }
  }

  fn parse_first_line(&self, line: &str) -> (String, String) {
    let mut splits = line.split(' ');
    let method = String::from(splits.next().unwrap());
    let path = String::from(splits.next().unwrap());
    (method, path)
  }

  fn parse_others(&self, lines: &mut Split<char>) -> HashMap<String, String> {
    lines.collect::<Vec<&str>>().iter()
        .map(|&s| {
          let mut split = s.splitn(2, ": ");
          (String::from(split.next().unwrap_or("")), String::from(split.next().unwrap_or("")))
        })
        .collect()
  }
}