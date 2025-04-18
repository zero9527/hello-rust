use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};

mod actix;
use actix::{app_state::AppState, hello_name, index, info, logo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(index::handle)
            .service(hello_name::handle)
            .service(info::handle_get)
            .service(info::handle_add)
            .service(info::handle_update)
            .service(info::handle_delete)
            .service(logo::handle_update)
            .default_service(web::to(|| HttpResponse::NotFound()))
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
