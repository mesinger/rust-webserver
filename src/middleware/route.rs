use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::{Middleware, MiddleWareResult};
use async_trait::async_trait;

pub type RouteMiddlewareResult = MiddleWareResult;

#[derive(Clone)]
pub struct RouteMiddleware {
  pub(crate) path: String,
  pub(crate) handler: Box<Arc<dyn Fn(&mut ServerContext) -> RouteMiddlewareResult + Send + Sync>>,
}

#[async_trait]
impl Middleware for RouteMiddleware {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult {
    if context.request.path != self.path {
      return Ok(());
    }

    (self.handler)(context)
  }
}
