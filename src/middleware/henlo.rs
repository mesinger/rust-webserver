use crate::core::context::{HttpCode, ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct HenloMiddleware {

}

impl Middleware for HenloMiddleware {
  fn action(&self, context: &mut ServerContext) {
    let message = "henlo shibe".to_string();
    context.response = Some(ServerHttpResponse {
      code: HttpCode::Ok,
      content_length: message.len() as i32,
      content: ServerHttpResponseContent::Text(message),
      content_type: "text/plain".to_string()
    })
  }
}