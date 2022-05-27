use std::sync::Arc;
use crate::core::context::ServerContext;

pub trait Middleware: Send + Sync {
  fn action(&self, context: &mut ServerContext);
}

pub type MiddlewarePipeline = Vec<Box<Arc<dyn Middleware>>>;