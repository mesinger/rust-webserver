use std::collections::VecDeque;
use std::sync::Arc;
use crate::core::context::ServerContext;
use async_trait::async_trait;

#[async_trait]
pub trait Middleware: Send + Sync {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline);
}

#[derive(Clone)]
pub struct MiddlewarePipeline {
  pub(crate) middlewares: VecDeque<Arc<dyn Middleware>>
}

impl MiddlewarePipeline {
  pub async fn next(&mut self, context: &mut ServerContext) {
    if let Some(middleware) = self.middlewares.pop_front() {
      middleware.action(context, self).await;
    }
  }
}
