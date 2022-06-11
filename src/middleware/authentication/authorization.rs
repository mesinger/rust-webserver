use std::collections::{HashMap};
use crate::core::context::{ServerContext, ServerHttpResponse};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;
use crate::core::authentication::ServerUser;

pub struct AuthorizationMiddleware {
  pub(crate) config: HashMap<String, String>
}

#[async_trait]
impl Middleware for AuthorizationMiddleware {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    let path_config = self.config.get(&context.request.path);

    if path_config.is_none() {
      pipeline.next(context).await;
      return;
    }

    let required_claim = path_config.unwrap();

    match &context.user {
      ServerUser::Anonymous => {
        context.response = Some(ServerHttpResponse::from_code(403));
        return;
      }
      ServerUser::Authenticated { claims, .. } => {
        if !claims.contains(required_claim) {
          context.response = Some(ServerHttpResponse::from_code(403));
          return;
        }
      }
    }

    pipeline.next(context).await;
  }
}
