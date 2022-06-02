use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Receiver};
use crate::core::context::{ServerContext, ServerHttpRequest, ServerHttpResponse};
use crate::core::middleware::{Middleware, MiddlewarePipeline};
use crate::middleware::http_parsing::HttpParsingMiddleware;
use crate::middleware::logging::LoggingMiddleware;


pub struct Server {
  pub(crate) address: String,
  pub(crate) middleware: MiddlewarePipeline,
}

impl Server {
  pub async fn run(&self, mut shutdown: Receiver<()>) {
    let listener = TcpListener::bind(self.address.as_str()).await.unwrap();

    println!("Running application");

    let (tx_process_finished, mut rx_process_finished) = tokio::sync::mpsc::channel(1);

    loop {
      let socket = tokio::select! {
        Ok((s, _)) = listener.accept() => {s},
        _ = shutdown.recv() => {
          break;
        }
      };

      tokio::spawn(Server::process(self.middleware.clone(), socket, tx_process_finished.clone()));
    }

    println!("Stopping application...");

    drop(tx_process_finished);
    let _ = rx_process_finished.recv().await;

    println!("Stopped application. All requests processed");
  }

  async fn process(mut p: MiddlewarePipeline, mut socket: TcpStream, _tx_finished: tokio::sync::mpsc::Sender<()>) {
    println!("{}", socket.peer_addr().unwrap());
    let msg = Server::incomming_request(&mut socket).await;

    let mut context = ServerContext {
      raw_message: msg,
      request: ServerHttpRequest::empty(),
      response: None,
    };

    p.next(&mut context).await;

    Server::send_response(&mut socket, context).await;
  }

  async fn incomming_request(socket: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    let _ = socket.read(&mut buffer).await.unwrap();

    String::from_utf8_lossy(&buffer).to_string()
  }

  async fn send_response(socket: &mut TcpStream, context: ServerContext) {
    if let Some(response) = context.response {
      socket.write_all(response.into_raw_http().as_slice()).await.unwrap();
    }
    socket.shutdown().await.unwrap();
  }
}
