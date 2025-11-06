use actix_web::{get, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
  user_id: u32,
  friend: String,
}

/**
 * Path 无需设置
 */
#[get("/user/{user_id}/{friend}")]
pub async fn greety(info: web::Path<Info>) -> Result<String> {
  Ok(format!(
    "welcome {}, user_id {}",
    info.friend, info.user_id
  ))
}