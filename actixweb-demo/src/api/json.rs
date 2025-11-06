use actix_web::{web, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Info {
  username: String,
  password: String,
}

#[derive(Serialize)]
struct LoginResponse {
  message: String,
  username: String,
}


/**
 * web::JSON<T> application/json
 */
#[post("/login")]
pub async fn login(web::Json(Info {username, password: _}): web::Json<Info>) -> web::Json<LoginResponse> {
  // 这里可以验证密码等逻辑
  web::Json(LoginResponse {
    message: format!("Login successful for user: {}", username),
    username,
  })
}