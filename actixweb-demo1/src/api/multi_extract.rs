use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, HttpResponse, Result};
use serde_json::json;

#[derive(MultipartForm)]
struct UploadForm {
  #[multipart(limit = "100MB")]
  file: TempFile,
  username: Text<String>,
  age: Text<u32>
}

#[post("/upload_multi_extract")]
async fn file_multi_extract(MultipartForm(form): MultipartForm<UploadForm>) -> Result<HttpResponse> {
  if let Some(file_name) = &form.file.file_name {
    let file_path = format!("./uploads_multi/{}", file_name);
    std::fs::create_dir_all("uploads_multi")?;
    let _ = form.file.file.persist(&file_path);
  }

  Ok(HttpResponse::Ok().json(json!({
    "username": form.username.as_str(),
    "age": *form.age,
    "file_name": form.file.file_name,
    "file_size": form.file.size
  })))
}