use crate::core::context::{HttpCode, ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct ErrorNotFoundMiddleware {

}

impl Middleware for ErrorNotFoundMiddleware {
  fn action(&self, context: &mut ServerContext) {
    match context.response {
      Some(_) => {}
      None => {
        context.response = Some(ServerHttpResponse {
          code: HttpCode::NotFound,
          content: ServerHttpResponseContent::Empty,
          content_length: 0,
          content_type: "text/plain".to_string()
        })
      }
    }
  }
}
