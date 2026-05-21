use actix_web::{HttpResponse, Scope, post, web};

pub fn containers_scope() -> Scope {
    web::scope("/containers").service(create)
}

#[post("/create")]
async fn create() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Created().finish())
}
