use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct RouteMiddleware {
  pub(crate) path: String,
  pub(crate) handler: Box<Arc<dyn Fn(&mut ServerContext) + Send + Sync>>,
}

#[async_trait]
impl Middleware for RouteMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    if context.request.path == self.path {
      (self.handler)(context);
    }

    pipeline.next(context).await;
  }
}
