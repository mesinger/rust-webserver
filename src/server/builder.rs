use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use crate::AsyncHandler;
use crate::core::context::ServerContext;
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use crate::middleware::authentication::authorization::AuthorizationMiddleware;
use crate::middleware::http_parsing::HttpParsingMiddleware;
use crate::middleware::logging::LoggingMiddleware;
use crate::middleware::route::{Route, RouteConfig, RouteHandler, RouteMiddleware};
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

  pub fn use_authorization(&mut self, config: HashMap<&str, &str>) {
    let middleware = Arc::new(AuthorizationMiddleware {
      config: config.into_iter().map(|(path, claim)| (path.to_string(), claim.to_string())).collect(),
    });
    self.middleware.push_back(middleware);
  }

  pub fn use_logging(&mut self) {
    let middleware = Arc::new(LoggingMiddleware {});
    self.middleware.push_back(middleware);
  }

  pub fn use_route(&mut self, routes: Vec<(&str, &str, RouteHandler)>) {
    let middleware = Arc::new(RouteMiddleware {
      routes: routes.into_iter().map(|(method, path, handler)| {
        (RouteConfig {method: method.to_string(), route: path.to_string()}, Route { handler })
      }).collect(),
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

pub fn map_get(path: &str, handler: impl Fn(&mut ServerContext) + Send + Sync + 'static) -> (&str, &str, RouteHandler) {
  ("GET", path, RouteHandler::Closure(Arc::new(handler)))
}

pub fn map_get_handler(path: &str, handler: impl AsyncHandler + 'static) -> (&str, &str, RouteHandler) {
  ("GET", path, RouteHandler::AsyncHandler(Arc::new(handler)))
}
