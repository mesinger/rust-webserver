use crate::core::context::{HttpCode, ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::{Middleware, MiddleWareResult};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ErrorNotFoundMiddleware {

}

#[async_trait]
impl Middleware for ErrorNotFoundMiddleware {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult {
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
    Ok(())
  }
}
