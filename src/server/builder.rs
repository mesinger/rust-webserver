use std::collections::VecDeque;
use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use crate::middleware::http_parsing::HttpParsingMiddleware;
use crate::middleware::logging::LoggingMiddleware;
use crate::middleware::route::{RouteMiddleware};
use crate::middleware::serve_dir::ServeDirMiddleware;
use crate::server::web_server::Server;

pub struct ServerBuilder {
  pub(crate) address: String,
  pub(crate) middleware: VecDeque<Arc<dyn Middleware>>,
}

impl ServerBuilder {
  pub fn build(self) -> Server {
    Server {
      address: self.address,
      middleware: MiddlewarePipeline {middlewares: self.middleware },
    }
  }
}

impl ServerBuilder {
  pub fn use_http_parsing(&mut self) {
    let middleware = Arc::new(HttpParsingMiddleware {});
    self.middleware.push_back(middleware);
  }

  pub fn use_authentication(&mut self, authentication_middleware: Arc<dyn Middleware>) {
    self.middleware.push_back(authentication_middleware);
  }

  pub fn use_logging(&mut self) {
    let middleware = Arc::new(LoggingMiddleware {});
    self.middleware.push_back(middleware);
  }

  pub fn use_route(&mut self, path: &'static str, handler: impl Fn(&mut ServerContext) + Send + Sync + 'static) {
    let middleware = Arc::new(RouteMiddleware {
      path: path.to_string(),
      handler: Box::new(Arc::new(handler)),
    });

    self.middleware.push_back(middleware);
  }

  pub fn use_serve_dir(&mut self, directory: &'static str) {
    let middleware = Arc::new(ServeDirMiddleware {
      directory_path: directory.to_string(),
    });

    self.middleware.push_back(middleware);
  }
}
