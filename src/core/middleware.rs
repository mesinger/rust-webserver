use std::sync::Arc;
use crate::core::context::ServerContext;
use async_trait::async_trait;

pub type MiddleWareResult = Result<(), ()>;

#[async_trait]
pub trait Middleware: Send + Sync {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult;
}

pub type MiddlewarePipeline = Vec<Box<Arc<dyn Middleware>>>;