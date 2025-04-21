use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn handle() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
