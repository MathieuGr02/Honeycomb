use std::{io, process::Command};

use actix_web::{HttpResponse, Scope, post, web};
use log::info;

use crate::container::Container;
pub fn containers_scope() -> Scope {
    web::scope("/containers").service(create)
}

#[post("/create")]
async fn create(container: web::Json<Container>) -> Result<HttpResponse, actix_web::Error> {
    info!("Creating container");
    let _ = Command::new("./target/release/honeycomb-runtime")
        .arg("child")
        .stderr(io::stderr())
        //.stdin(io::stdin())
        .stdout(io::stdout())
        .output()
        .map_err(|e| panic!("{}", e));
    Ok(HttpResponse::Created().finish())
}
