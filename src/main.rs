use std::collections::HashSet;
use std::sync::Arc;
use tokio::signal;
use async_trait::async_trait;
use tokio::sync::{broadcast};
use crate::core::authentication::{AuthenticationService, ServerUser};
use crate::core::context::ServerContext;
use crate::middleware::authentication::basic_authentication::BasicAuthenticationMiddleware;
use crate::middleware::route::AsyncHandler;
use crate::server::builder::{map_get, map_get_handler};

mod server;
mod core;
mod middleware;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let mut app = server::app::listen("127.0.0.1:8080");
  app.use_http_parsing();

  app.use_authentication(Arc::new(BasicAuthenticationMiddleware {
    authentication_service: Arc::new(MockedAuthenticationService {}),
    paths: ["/contact.html", "/shibe"].into(),
  }));

  app.use_authorization([
    ("/contact.html", "doge"),
  ].into());

  app.use_route([
    map_get("/shibe", |ctx: &mut ServerContext| ctx.set_response("you are a shibe")),
    map_get_handler("/doge", DogeRouteHandler {}),
  ].into());

  app.use_serve_dir("asset");

  app.use_logging();

  let app = app.build();

  let (tx_shutdown, rx_shutdown) = broadcast::channel(1);

  let app = tokio::spawn(async move {
    app.run(rx_shutdown).await;
  });

  signal::ctrl_c().await.expect("Cannot receive CTRL+C");

  tx_shutdown.send(()).unwrap();

  app.await.unwrap();
}

struct MockedAuthenticationService;

#[async_trait]
impl AuthenticationService for MockedAuthenticationService {
  async fn authenticate_user_and_password<'a>(&'a self, user: &'a str, password: &'a str) -> Result<ServerUser, ()> {
    match password {
      "password" => Ok(ServerUser::Authenticated {
        id: user.to_string(),
        email: Some(format!("{}@email.com", user)),
        claims: HashSet::from(["doge".to_string()]),
      }),
      _ => Err(())
    }
  }
}

struct DogeRouteHandler{}

#[async_trait]
impl AsyncHandler for DogeRouteHandler {
  async fn handle(&self, context: &mut ServerContext) {
    context.set_response("I am the doge handler");
  }
}
