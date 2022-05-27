use std::fs;
use crate::core::context::{HttpCode, ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct ServeDirMiddleware {
  pub(crate) directory_path: String,
}

impl Middleware for ServeDirMiddleware {
  fn action(&self, context: &mut ServerContext) {
    let mut file_path = context.request.path.as_str();
    if file_path == "/" {
      file_path = "/index.html";
    }

    let mut content_type;
    if file_path.ends_with("html") {
      content_type = "text/html";
    } else if file_path.ends_with("png") {
      content_type = "image/png"
    } else {
      content_type = "text/plain";
    }

    if let Ok(content) = fs::read_to_string(format!("{}{}", self.directory_path, file_path)) {

      context.response = Some(ServerHttpResponse {
        code: HttpCode::Ok,
        content_length: content.len() as i32,
        content: ServerHttpResponseContent::Text(content),
        content_type: content_type.to_string(),
      });

      return;
    }

    if let Ok(bytes) = fs::read(format!("{}{}", self.directory_path, file_path)) {
      context.response = Some(ServerHttpResponse {
        code: HttpCode::Ok,
        content_length: bytes.len() as i32,
        content: ServerHttpResponseContent::Binary(bytes),
        content_type: content_type.to_string(),
      });
    }
  }
}
