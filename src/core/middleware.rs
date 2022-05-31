use std::sync::Arc;
use crate::core::context::ServerContext;
use async_trait::async_trait;

pub type MiddleWareResult = Result<(), ()>;

#[async_trait]
pub trait Middleware: Send + Sync {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult;
  async fn next(&self, context: &mut ServerContext) -> MiddleWareResult;
  fn set_next(&mut self, next: Arc<dyn Middleware>);
}

pub type MiddlewarePipeline = Vec<Arc<dyn Middleware>>;