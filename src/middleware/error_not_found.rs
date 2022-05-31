use std::sync::Arc;
use crate::core::context::{HttpCode, ServerContext, ServerHttpResponse, ServerHttpResponseContent};
use crate::core::middleware::{Middleware, MiddleWareResult};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ErrorNotFoundMiddleware {
  pub(crate) next: Option<Arc<dyn Middleware>>,
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

    self.next(context).await;

    Ok(())
  }

  async fn next(&self, context: &mut ServerContext) -> MiddleWareResult {
    if let Some(ref next) = self.next {
      return next.action(context).await;
    }

    Ok(())
  }

  fn set_next(&mut self, next: Arc<dyn Middleware>) {
    self.next = Some(next);
  }
}
