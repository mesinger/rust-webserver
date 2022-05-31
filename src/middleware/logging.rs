use std::sync::Arc;
use crate::core::context::{ServerContext, ServerHttpRequest};
use crate::core::middleware::{Middleware, MiddleWareResult};
use async_trait::async_trait;

#[derive(Clone)]
pub struct LoggingMiddleware {
  pub(crate) next: Option<Arc<dyn Middleware>>,
}

#[async_trait]
impl Middleware for LoggingMiddleware {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult {
    self.log_http(&context.request);

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

impl LoggingMiddleware {
  fn log_http(&self, req: &ServerHttpRequest) {
    let msg = format!("chatty (HTTP):\n{} {}\nHost: {}\nUser-Agent: {}\n", req.method, req.path, req.host, req.user_agent);
    println!("{}", msg);
  }
}