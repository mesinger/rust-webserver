use std::collections::HashSet;
use std::sync::Arc;
use tokio::signal;
use async_trait::async_trait;
use tokio::sync::{broadcast};
use crate::core::authentication::{AuthenticationService};
use crate::middleware::authentication::basic_authentication::BasicAuthenticationMiddleware;

mod server;
mod core;
mod middleware;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let mut app = server::app::listen("127.0.0.1:8080");
  app.use_http_parsing();
  app.use_authentication(Arc::new(BasicAuthenticationMiddleware {
    authentication_service: Arc::new(MockedAuthenticationService {}),
    paths: HashSet::from(["/contact.html"]),
  }));
  app.use_route("/shibe", |ctx| ctx.set_response("you are a shibe"));
  app.use_serve_dir("asset");
  app.use_logging();

  let app = app.build();

  let (tx_shutdown, rx_shutdown) = broadcast::channel(1);

  let app = tokio::spawn(async move {
    app.run(rx_shutdown).await;
  });

  match signal::ctrl_c().await {
    Ok(()) => {}
    Err(err) => {
      eprintln!("Unable to receive ctrl_c {}", err);
    }
  }

  tx_shutdown.send(()).unwrap();

  app.await.unwrap();
}

struct MockedAuthenticationService;

#[async_trait]
impl AuthenticationService for MockedAuthenticationService {
  async fn authenticate_user_and_password<'a>(&'a self, _user: &'a str, password: &'a str) -> Result<(), ()> {
    match password {
      "password" => Ok(()),
      _ => Err(())
    }
  }
}
