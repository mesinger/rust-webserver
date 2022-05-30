use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Receiver};
use crate::core::context::{ServerContext, ServerHttpRequest, ServerHttpResponse};
use crate::core::middleware::{MiddlewarePipeline};
use crate::middleware::error_not_found::ErrorNotFoundMiddleware;
use crate::middleware::http_parsing::HttpParsingMiddleware;
use crate::middleware::logging::LoggingMiddleware;
use crate::middleware::route::{RouteMiddleware, RouteMiddlewareResult};
use crate::middleware::serve_dir::ServeDirMiddleware;


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

  async fn process(p: MiddlewarePipeline, mut socket: TcpStream, _tx_finished: tokio::sync::mpsc::Sender<()>) {
    println!("{}", socket.peer_addr().unwrap());
    let msg = Server::incomming_request(&mut socket).await;

    let mut context = ServerContext {
      raw_message: msg,
      request: ServerHttpRequest::empty(),
      response: None,
    };

    let pipeline_result = Server::run_pipeline(p, &mut context).await;

    if pipeline_result.is_err() {
      context.response = Some(ServerHttpResponse::internal_error());
    }

    Server::send_response(&mut socket, context).await;
  }

  async fn incomming_request(socket: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    let _ = socket.read(&mut buffer).await.unwrap();

    String::from_utf8_lossy(&buffer).to_string()
  }

  async fn run_pipeline(p: MiddlewarePipeline, context: &mut ServerContext) -> Result<(), ()> {
    for middle_ware in p.into_iter() {
      let middle_ware_result = middle_ware.action(context).await;
      if middle_ware_result.is_err() {
        return Err(())
      }
    }

    Ok(())
  }

  async fn send_response(socket: &mut TcpStream, context: ServerContext) {
    if let Some(response) = context.response {
      socket.write_all(response.into_raw_http().as_slice()).await.unwrap();
    }
    socket.shutdown().await.unwrap();
  }
}

impl Server {
  pub fn use_http_parsing(&mut self) {
    self.middleware.push(Box::new(Arc::new(HttpParsingMiddleware {})));
  }

  pub fn use_logging(&mut self) {
    self.middleware.push(Box::new(Arc::new(LoggingMiddleware {})));
  }

  pub fn use_route(&mut self, path: &'static str, handler: impl Fn(&mut ServerContext) -> RouteMiddlewareResult + Send + Sync + 'static) {
    self.middleware.push(Box::new(Arc::new(RouteMiddleware {
      path: path.to_string(),
      handler: Box::new(Arc::new(handler)),
    })));
  }

  pub fn use_error_handling(&mut self) {
    self.middleware.push(Box::new(Arc::new(ErrorNotFoundMiddleware {})));
  }

  pub fn use_serve_dir(&mut self, directory: &'static str) {
    self.middleware.push(Box::new(Arc::new(ServeDirMiddleware {
      directory_path: directory.to_string(),
    })));
  }
}


