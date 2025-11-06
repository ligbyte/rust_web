use actix_web::{web, App, HttpServer, HttpResponse};

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
            .service(greety)
            .service(login)
            .service(get_user_name)
            .service(index)
            .service(upload_mix)
            .service(file_multi_extract)
            .service(error_handle)
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("Hi")}))
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
