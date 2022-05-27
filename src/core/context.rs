pub struct ServerContext {
  pub(crate) raw_message: String,
  pub(crate) request: ServerHttpRequest,
  pub(crate) response: Option<ServerHttpResponse>,
}

impl ServerContext {
  pub fn set_response(&mut self, content: &'static str) {
    self.response = Some(ServerHttpResponse {
      code: HttpCode::Ok,
      content_length: content.len() as i32,
      content: ServerHttpResponseContent::Text(content.to_string()),
      content_type: "text/plain".to_string(),
    });
  }
}

pub struct ServerHttpRequest {
  pub(crate) method: String,
  pub(crate) path: String,
  pub(crate) host: String,
  pub(crate) user_agent: String,
}

impl ServerHttpRequest {
  pub fn empty() -> Self {
    ServerHttpRequest {
      method: "".to_string(),
      path: "".to_string(),
      host: "".to_string(),
      user_agent: "".to_string()
    }
  }
}

pub struct ServerHttpResponse {
  pub(crate) code: HttpCode,
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
  pub fn to_raw_http(&self) -> Vec<u8> {
    let mut content = match &self.content {
      ServerHttpResponseContent::Text(text) => {text.clone().into_bytes()}
      ServerHttpResponseContent::Binary(data) => {data.to_vec()},
      ServerHttpResponseContent::Empty => vec![]
    };

    let response = format!("HTTP/1.1 {} {}
Server: chatty
Content-Length: {}
Content-Type: {}
Connection: Closed

", self.code.to_status_code(), self.code.to_reason_phrase(), self.content_length, self.content_type);

    let mut data = response.into_bytes();
    data.append(&mut content);
    data
  }
}

pub enum HttpCode {
  Ok,
  NotFound,
  InternalServerError,
}

impl HttpCode {
  fn to_status_code(&self) -> u16 {
    match self {
      HttpCode::Ok => 200,
      HttpCode::NotFound => 404,
      HttpCode::InternalServerError => 500,
    }
  }

  fn to_reason_phrase(&self) -> &'static str {
    match self {
      HttpCode::Ok => "OK",
      HttpCode::NotFound => "Not Found",
      HttpCode::InternalServerError => "Internal Server Error",
    }
  }
}
