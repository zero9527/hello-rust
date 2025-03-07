use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello/{name}")]
pub async fn handle(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}
