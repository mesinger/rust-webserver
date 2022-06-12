use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use crate::core::context::ServerContext;
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

#[derive(Clone)]
pub struct RouteMiddleware {
  pub(crate) routes: HashMap<RouteConfig, Route>
}

#[async_trait]
impl Middleware for RouteMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    let route = self.routes.get(&RouteConfig {
      method: context.request.method.clone(),
      route: context.request.path.clone()
    });

    if let Some(route) = route {
      route.invoke(context).await;
    }

    pipeline.next(context).await;
  }
}

#[derive(Clone)]
pub struct RouteConfig {
  pub(crate) method: String,
  pub(crate) route: String,
}

#[derive(Clone)]
pub struct Route {
  pub(crate) handler: RouteHandler,
}

impl Route {
  async fn invoke(&self, context: &mut ServerContext) {
    match &self.handler {
      RouteHandler::Closure(closure) => {
        (closure)(context)
      }
      RouteHandler::AsyncHandler(handler) => {
        handler.handle(context).await;
      }
    }
  }
}

#[derive(Clone)]
pub enum RouteHandler {
  Closure(Arc<dyn Fn(&mut ServerContext) + Send + Sync>),
  AsyncHandler(Arc<dyn AsyncHandler>),
}

#[async_trait]
pub trait AsyncHandler: Send + Sync {
  async fn handle(&self, context: &mut ServerContext);
}

impl PartialEq<Self> for RouteConfig {
  fn eq(&self, other: &Self) -> bool {
    self.method == other.method && self.route == other.route
  }
}

impl Eq for RouteConfig { }

impl Hash for RouteConfig {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.method.hash(state);
    self.route.hash(state);
  }
}
