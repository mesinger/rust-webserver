use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::Middleware;
use crate::middleware::error_not_found::ErrorNotFoundMiddleware;
use crate::middleware::http_parsing::HttpParsingMiddleware;
use crate::middleware::logging::LoggingMiddleware;
use crate::middleware::route::{RouteMiddleware, RouteMiddlewareResult};
use crate::middleware::serve_dir::ServeDirMiddleware;
use crate::server::web_server::Server;

pub struct ServerBuilder {
  pub(crate) address: String,
  pub(crate) middleware: Vec<Box<dyn Middleware>>,
}

impl ServerBuilder {
  pub fn build(self) -> Server {
    Server {
      address: self.address,
      middleware: self.middleware.into_iter().map(|m| Arc::from(m)).collect()
    }
  }
}

impl ServerBuilder {
  pub fn use_http_parsing(&mut self) {
    let middleware = Box::new(HttpParsingMiddleware {
      next: None,
    });

    self.connect_to_previous_middleware(middleware.clone());

    self.middleware.push(middleware);
  }

  pub fn use_logging(&mut self) {
    let middleware = Box::new(LoggingMiddleware {
      next: None,
    });

    self.connect_to_previous_middleware(middleware.clone());

    self.middleware.push(middleware);
  }

  pub fn use_route(&mut self, path: &'static str, handler: impl Fn(&mut ServerContext) -> RouteMiddlewareResult + Send + Sync + 'static) {
    let middleware = Box::new(RouteMiddleware {
      path: path.to_string(),
      handler: Box::new(Arc::new(handler)),
      next: None
    });

    self.connect_to_previous_middleware(middleware.clone());

    self.middleware.push(middleware);
  }

  pub fn use_error_handling(&mut self) {
    let middleware = Box::new(ErrorNotFoundMiddleware {
      next: None,
    });

    self.connect_to_previous_middleware(middleware.clone());

    self.middleware.push(middleware);
  }

  pub fn use_serve_dir(&mut self, directory: &'static str) {
    let middleware = Box::new(ServeDirMiddleware {
      directory_path: directory.to_string(),
      next: None
    });

    self.connect_to_previous_middleware(middleware.clone());

    self.middleware.push(middleware);
  }
}

impl ServerBuilder {
  fn connect_to_previous_middleware(&mut self, next: Box<dyn Middleware>) {
    if let Some(middleware) = self.middleware.last_mut() {
      // middleware.set_next(next);
    }
  }
}
