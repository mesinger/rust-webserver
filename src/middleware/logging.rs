
use crate::core::context::{ServerContext};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct LoggingMiddleware {
}

#[async_trait]
impl Middleware for LoggingMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    println!("{}", context);
    pipeline.next(context).await;
  }
}