use actix_web::{App, HttpServer, web};
use log::{error, info};
use tracing::{level_filters::LevelFilter, trace};

use honeycomb_engine::{logger::init_logger, routes::containers::containers_scope};

const SOCKET: &str = "./honeycomb.socket";

#[actix_web::main]
async fn main() {
    init_logger(LevelFilter::TRACE);

    HttpServer::new(|| App::new().service(containers_scope()))
        .bind_uds(SOCKET)
        .unwrap()
        .run()
        .await;
}
