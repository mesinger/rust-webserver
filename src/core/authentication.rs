use crate::core::middleware::Middleware;
use async_trait::async_trait;

#[async_trait]
pub trait AuthenticationMiddleware: Middleware {
  async fn authenticate<'a>(&'a self, authentication_value: &'a str) -> Result<(), ()>;
}

#[async_trait]
pub trait AuthenticationService: Send + Sync {
  async fn authenticate_user_and_password<'a>(&'a self, user: &'a str, password: &'a str) -> Result<(), ()>;
}
