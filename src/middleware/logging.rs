use crate::core::context::{ServerContext, ServerHttpRequest};
use crate::core::middleware::{Middleware, MiddleWareResult};
use async_trait::async_trait;

#[derive(Clone)]
pub struct LoggingMiddleware {

}

#[async_trait]
impl Middleware for LoggingMiddleware {
  async fn action(&self, context: &mut ServerContext) -> MiddleWareResult {
    self.log_http(&context.request);
    Ok(())
  }
}

impl LoggingMiddleware {
  fn log_http(&self, req: &ServerHttpRequest) {
    let msg = format!("chatty (HTTP):\n{} {}\nHost: {}\nUser-Agent: {}\n", req.method, req.path, req.host, req.user_agent);
    println!("{}", msg);
  }
}