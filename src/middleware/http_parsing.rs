use std::collections::HashMap;
use core::str::Split;

use crate::core::context::{ServerContext, ServerHttpRequest, ServerHttpResponse};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct HttpParsingMiddleware;

#[async_trait]
impl Middleware for HttpParsingMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    let parsed_request = self.parse_request(&context.raw_message);

    if parsed_request.is_none() {
      context.response = Some(ServerHttpResponse::internal_error());
      return;
    }

    context.request = parsed_request.unwrap();
    pipeline.next(context).await;

    if context.response.is_none() {
      context.response = Some(ServerHttpResponse::not_found());
    }
  }
}

impl HttpParsingMiddleware {
  fn parse_request(&self, content: &str) -> Option<ServerHttpRequest> {
    let mut lines = content.split('\n');
    let (method, path) = self.parse_method_and_path(lines.next()?)?;

    let headers = self.parse_headers(&mut lines);

    Some(ServerHttpRequest {
      method,
      path,
      headers,
    })
  }

  fn parse_method_and_path(&self, line: &str) -> Option<(String, String)> {
    let mut splits = line.split(' ');
    let method = String::from(splits.next()?);
    let path = String::from(splits.next()?);
    Some((method, path))
  }

  fn parse_headers(&self, lines: &mut Split<char>) -> HashMap<String, String> {
    lines.collect::<Vec<&str>>().iter()
        .map(|&s| {
          let mut split = s.splitn(2, ": ");
          (String::from(split.next().unwrap_or("")), String::from(split.next().unwrap_or("")))
        })
        .collect()
  }
}