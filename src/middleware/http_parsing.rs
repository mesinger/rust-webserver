use std::collections::HashMap;
use core::str::Split;
use std::sync::Arc;
use crate::core::context::{ServerContext, ServerHttpRequest, ServerHttpResponse};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct HttpParsingMiddleware {
}

#[async_trait]
impl Middleware for HttpParsingMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    let req = self.parse_request(&context.raw_message);
    context.request = req;

    pipeline.next(context).await;

    if context.response.is_none() {
      context.response = Some(ServerHttpResponse::internal_error());
    }
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
    println!("{}", line);
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