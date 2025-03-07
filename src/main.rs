use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer};

mod actix;
use actix::{app_state::AppState, hello_name, index, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let app_state = Mutex::new(AppState::new());
        App::new()
            .app_data(web::Data::new(app_state).clone())
            .service(index::handle)
            .service(hello_name::handle)
            .service(info::handle_get)
            .service(info::handle_add)
            .service(info::handle_update)
            .service(info::handle_delete)
            .default_service(web::to(|| HttpResponse::NotFound()))
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
