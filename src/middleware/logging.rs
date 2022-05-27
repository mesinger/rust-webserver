use crate::core::context::{ServerContext, ServerHttpRequest};
use crate::core::middleware::Middleware;

#[derive(Clone)]
pub struct LoggingMiddleware {

}

impl Middleware for LoggingMiddleware {
  fn action(&self, context: &mut ServerContext) {
    self.log_http(&context.request)
  }
}

impl LoggingMiddleware {
  fn log_http(&self, req: &ServerHttpRequest) {
    let msg = format!("chatty (HTTP):\n{} {}\nHost: {}\nUser-Agent: {}\n", req.method, req.path, req.host, req.user_agent);
    println!("{}", msg);
  }
}