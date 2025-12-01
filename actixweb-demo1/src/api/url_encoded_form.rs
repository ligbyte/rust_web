use actix_web::{post, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
  username: String
}

/**
 * web::Form<T> 对应前端 application/x-www-form-urlencoded
 */
#[post("/url-encoded-form")]
async fn index(form: web::Form<FormData>) -> Result<String> {
  println!("form: {}", form.username);
  Ok(format!("Welcome {}", form.username))
}