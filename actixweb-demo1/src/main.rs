use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;

mod api;
use api::greety;
use api::get_user_name;
use api::login;
use api::index;
use api::upload_mix;
use api::file_multi_extract;
use api::error_handle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                .allow_any_origin()  // 允许所有来源（仅用于开发环境）
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
            )
            .service(greety)
            .service(login)
            .service(get_user_name)
            .service(index)
            .service(upload_mix)
            .service(file_multi_extract)
            .service(error_handle)
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("Hi")}))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
