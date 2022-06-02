use tokio::signal;

use tokio::sync::{broadcast};

mod server;
mod core;
mod middleware;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let mut app = server::app::listen("127.0.0.1:8080");
  app.use_http_parsing();
  app.use_logging();
  app.use_route("/shibe", |ctx| ctx.set_response("you are a shibe"));
  app.use_serve_dir("asset");

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
