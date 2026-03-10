#[allow(warnings)]
mod Model;
#[allow(warnings)]
mod Tools;
#[allow(warnings)]
mod Controller;

use actix_web::{App, HttpServer, middleware};
use crate::Model::model::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    // ip:port
    let server = format!("{}:{}",
         config.website.address,
         config.website.port
    );

    // 工作线程数
    let workers = config.website.workers.parse().unwrap_or(1);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Controller::page::index)

            // 管理员的操作
            .service(Controller::admin::adminLogin)
            .service(Controller::admin::adminUpdate)
            .service(Controller::admin::adminAddPage)
            .service(Controller::admin::adminDelPageId)

            // 对网站的展示信息进行操作
            .service(Controller::website::websiteFriend)
            .service(Controller::website::websiteIndexMessageShow)
            .service(Controller::website::websiteIndexMessageUpdate)

            // 对博客进行操作
            .service(Controller::blog::blogList)
            .service(Controller::blog::hotPage)
            .service(Controller::blog::blogCommitIdAdd)
            .service(Controller::blog::blogCommitIdShow)

            // 访客能进行的操作
            .service(Controller::vistor::vistorAddCommit)
            .service(Controller::vistor::vistorAddFriend)
    })
    .bind(server)?
    .workers(workers)
    .run()
    .await
}