use std::collections::HashMap;
use std::fs;

use crate::core::context::{ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ServeDirMiddleware {
  pub(crate) directory_path: String,
}

#[async_trait]
impl Middleware for ServeDirMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    let mut file_path = context.request.path.as_str();
    if file_path == "/" {
      file_path = "/index.html";
    }

    let file_path = file_path.replace("../", "");

    if let Ok(bytes) = fs::read(format!("{}{}", self.directory_path, file_path)) {

      let content_type = self.content_type_for_file_name(&file_path);

      context.response = Some(ServerHttpResponse {
        code: 200,
        content_length: bytes.len() as i32,
        content: ServerHttpResponseContent::Binary(bytes),
        content_type: content_type.to_string(),
        headers: HashMap::new(),
      });
    }

    pipeline.next(context).await;
  }
}

impl ServeDirMiddleware {
  fn content_type_for_file_name(&self, file_name: &str) -> &'static str {
    let content_type;
    if file_name.ends_with("html") {
      content_type = "text/html";
    } else if file_name.ends_with("png") {
      content_type = "image/png"
    } else {
      content_type = "text/plain";
    }

    content_type
  }
}
