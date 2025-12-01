use actix_web::{Result, post, HttpResponse};
use futures_util::stream::StreamExt;
use actix_multipart:: Multipart;
use std::io::Write;

#[post("/upload_with_fields")]
async fn upload_mix(mut payload: Multipart) -> Result<HttpResponse> {
  let mut username = String::new();
  let mut age = String::new();
  let mut file_name = String::new();
  let mut file_data = Vec::new();

  while let Some(item) = payload.next().await {
    let mut field = item?;
    let field_name = field.name().unwrap_or("").to_string();

    match field_name.as_str() {
      "username" => {
        while let Some(chunk) = field.next().await {
          username.push_str(&String::from_utf8_lossy(&chunk?));
        }
      },
      "age" => {
        while let Some(chunk) = field.next().await {
          age.push_str(&String::from_utf8_lossy(&chunk?));
        }
      },
      "file" => {
        if let Some(content_disposition) = field.content_disposition() {
          file_name = content_disposition
                      .get_filename()
                      .unwrap_or("unknown")
                      .to_string()
        }

        while let Some(chunk) = field.next().await {
          file_data.extend_from_slice(&chunk?);
        }

        let file_path = format!("./uploads/{}", file_name);
        std::fs::create_dir_all("./uploads")?;
        let mut f = std::fs::File::create(&file_path)?;
        f.write_all(&file_data)?;
      },
      _ => {}
    }
  }

  Ok(HttpResponse::Ok().json(serde_json::json!({
    "username": username,
    "age": age,
    "file_name": file_name,
    "file_size": file_data.len()
  })))
}