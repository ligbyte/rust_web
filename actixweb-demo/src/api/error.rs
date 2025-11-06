use actix_web::{error, get, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error)]
enum MyError {
  #[display(fmt = "internal error")]
  InternalError,

  #[display(fmt = "bad request")]
  BadRequest,

  #[display(fmt = "timeout")]
  Timeout
}

#[derive(Serialize)]
struct ErrorResponse {
  status: u16,
  msg: String,
}

impl error::ResponseError for MyError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
        .json(ErrorResponse {
          status: self.status_code().as_u16(),
          msg: self.to_string()
        })
  }

  fn status_code(&self) -> StatusCode {
      match *self {
          MyError::BadRequest => StatusCode::BAD_REQUEST,
          MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
          MyError::Timeout => StatusCode::GATEWAY_TIMEOUT
      }
  }
}

#[get("/error")]
pub async fn error_handle() -> Result<&'static str, MyError> {
  Err(MyError::InternalError)
}
