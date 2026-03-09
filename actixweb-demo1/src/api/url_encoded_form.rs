use actix_web::{post, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
  username: String
}

/**
<form action="/api/submit" method="post">
  <!-- 表单默认使用此编码 -->
  <input name="username" value="张三">
  <input name="age" value="20">
  <button>提交</button>
</form>

*/

/**
 * web::Form<T> 对应前端 application/x-www-form-urlencoded
 */
#[post("/url-encoded-form")]
async fn index(form: web::Form<FormData>) -> Result<String> {
  println!("form: {}", form.username);
  Ok(format!("Welcome {}", form.username))
}