
use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct RouteMiddleware {
  pub(crate) path: String,
  pub(crate) handler: Box<Arc<dyn Fn(&mut ServerContext) + Send + Sync>>,
}

impl Middleware for RouteMiddleware {
  fn action(&self, context: &mut ServerContext) {
    if context.request.path != self.path {
      return;
    }

    (self.handler)(context);
  }
}
