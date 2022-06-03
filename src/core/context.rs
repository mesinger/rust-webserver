use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub struct ServerContext {
  pub(crate) raw_message: String,
  pub(crate) request: ServerHttpRequest,
  pub(crate) response: Option<ServerHttpResponse>,
}

impl ServerContext {
  pub fn set_response(&mut self, content: &'static str) {
    self.response = Some(ServerHttpResponse {
      code: 200,
      content_length: content.len() as i32,
      content: ServerHttpResponseContent::Text(content.to_string()),
      content_type: "text/plain".to_string(),
    });
  }
}

impl Display for ServerContext {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.response.is_some() {
      write!(f, "Request: {}, Response: {}", self.request, self.response.as_ref().unwrap())
    } else {
      write!(f, "Request: {}, Response: Empty", self.request)
    }
  }
}

pub struct ServerHttpRequest {
  pub(crate) method: String,
  pub(crate) path: String,
  pub(crate) headers: HashMap<String, String>,
}

impl ServerHttpRequest {
  pub fn empty() -> Self {
    ServerHttpRequest {
      method: "".to_string(),
      path: "".to_string(),
      headers: HashMap::new(),
    }
  }
}

impl Display for ServerHttpRequest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.method, self.path)
  }
}

impl Debug for ServerHttpRequest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} HTTP/1.1\n{}", self.method, self.path, self.headers.iter().map(|(name, value)| format!("{}: {}", name, value)).collect::<Vec<String>>().join("\n"))
  }
}

pub struct ServerHttpResponse {
  pub(crate) code: u16,
  pub(crate) content: ServerHttpResponseContent,
  pub(crate) content_length: i32,
  pub(crate) content_type: String,
}

pub enum ServerHttpResponseContent {
  Text(String),
  Binary(Vec<u8>),
  Empty,
}

impl ServerHttpResponse {
  pub fn internal_error() -> Self {
    ServerHttpResponse::from_code(500)
  }

  pub fn not_found() -> Self {
    ServerHttpResponse::from_code(404)
  }

  fn from_code(code: u16) -> Self {
    ServerHttpResponse {
      code,
      content: ServerHttpResponseContent::Empty,
      content_length: 0,
      content_type: "text/plain".to_string()
    }
  }

  pub fn into_raw_http(self) -> Vec<u8> {
    let mut content = match self.content {
      ServerHttpResponseContent::Text(text) => {text.into_bytes()}
      ServerHttpResponseContent::Binary(data) => {data},
      ServerHttpResponseContent::Empty => vec![]
    };

    let response = format!("HTTP/1.1 {} {}
Server: chatty
Content-Length: {}
Content-Type: {}
Connection: Closed

", self.code, ServerHttpResponse::code_to_reason_phrase(self.code), self.content_length, self.content_type);

    let mut data = response.into_bytes();
    data.append(&mut content);
    data
  }

  fn code_to_reason_phrase(code: u16) -> &'static str {
    match code {
      200 => "OK",
      404 => "Not Found",
      500 => "Internal Server Error",
      _ => panic!("Invalid http code")
    }
  }
}

impl Display for ServerHttpResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "HTTP {} {}", self.code, ServerHttpResponse::code_to_reason_phrase(self.code))
  }
}
