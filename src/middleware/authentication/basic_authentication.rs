use std::collections::HashSet;
use std::sync::Arc;
use crate::core::authentication::{AuthenticationMiddleware, AuthenticationService};
use crate::core::context::{ServerContext, ServerHttpResponse};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use async_trait::async_trait;

pub struct BasicAuthenticationMiddleware<T> where T: AuthenticationService {
  pub(crate) authentication_service: Arc<T>,
  pub(crate) paths: HashSet<&'static str>,
}

#[async_trait]
impl<T: AuthenticationService> Middleware for BasicAuthenticationMiddleware<T> {
  async fn action(&self, context: &mut ServerContext, pipeline: &mut MiddlewarePipeline) {
    if !self.paths.contains(context.request.path.as_str()) {
      pipeline.next(context).await;
      return;
    }

    let authorization_header = context.request.headers.get("Authorization");

    if authorization_header.is_none() {
      context.response = Some(BasicAuthenticationMiddleware::<T>::build_unauthorized_response(context.request.path.as_str()));
      return;
    }

    let authentication_result = self.authenticate(authorization_header.unwrap().as_str()).await;

    if authentication_result.is_err() {
      context.response = Some(BasicAuthenticationMiddleware::<T>::build_unauthorized_response(context.request.path.as_str()));
      return;
    }

    pipeline.next(context).await;
  }
}

#[async_trait]
impl<T: AuthenticationService> AuthenticationMiddleware for BasicAuthenticationMiddleware<T> {
  async fn authenticate<'a>(&'a self, authentication_value: &'a str) -> Result<(), ()> {
    let mut splits = authentication_value.splitn(2, ' ');
    let basic: Option<&str> = splits.next();
    let sequence: Option<&str> = splits.next();

    if basic.is_none() || basic.unwrap() != "Basic" {
      return Err(());
    }

    if sequence.is_none() {
      return Err(());
    }

    let sequence = sequence.unwrap();
    let decoded_sequence_raw = base64::decode(sequence);

    if decoded_sequence_raw.is_err() {
      return Err(());
    }

    let decoded_sequence = String::from_utf8(decoded_sequence_raw.unwrap());

    if decoded_sequence.is_err() {
      return Err(());
    }

    let decoded_sequence = decoded_sequence.unwrap();

    let mut splits = decoded_sequence.splitn(2, ':');
    let username: Option<&str> = splits.next();
    let password: Option<&str> = splits.next();

    if username.is_none() || password.is_none() {
      return Err(());
    }

    let authentication_result = self.authentication_service.authenticate_user_and_password(username.unwrap(), password.unwrap()).await;
    authentication_result
  }
}

impl<T: AuthenticationService> BasicAuthenticationMiddleware<T> {
  fn build_unauthorized_response(realm: &str) -> ServerHttpResponse {
    let mut response = ServerHttpResponse::from_code(401);
    response.headers.insert("WWW-Authenticate".to_string(), format!("Basic realm=\"{}\" charset=\"UTF-8\"", realm));
    response
  }
}
