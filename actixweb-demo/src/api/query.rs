use actix_web::{get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
  username: String,
}

/**
 * web::Query<T> URL参数
 */
#[get("/user/name")]
pub async fn get_user_name(info: web::Query<Info>) -> String {
  let web::Query(Info { username }) = info;
  format!("welcome {}!", username)
}