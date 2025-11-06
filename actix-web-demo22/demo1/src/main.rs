use actix_web::{web, get, HttpServer, App};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    println!("🚀 Service started at port: {} 🚀", "http://127.0.0.1:8080/");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello 27 {app_name}!")
}
