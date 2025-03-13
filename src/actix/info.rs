use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::actix::app_state::{AppState, Info};

#[get("/get/info/{name}")]
async fn handle_get(name: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    println!("\ndata: {:?}", name);
    println!("app_state.info: {:?}", app_state.info.lock().unwrap());
    if let Ok(Some(info)) = app_state.get(&name) {
        return HttpResponse::Ok().json(info);
    }

    HttpResponse::BadRequest().body(format!("name: {} 不存在", &name))
}

#[post("/add/info")]
async fn handle_add(req_body: web::Json<Info>, app_state: web::Data<AppState>) -> impl Responder {
    println!("\n[/add/info] req_body: {:?}", req_body);
    if let Err(e) = app_state.add(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[post("/update/info")]
async fn handle_update(
    req_body: web::Json<Info>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    println!("\n[/update/info] req_body: {:?}", req_body);
    if let Err(e) = app_state.update(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[delete("/delete/{name}")]
async fn handle_delete(name: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    println!("\n[/delete/{name}] name: {:?}", name);
    if let Err(e) = app_state.remove(name.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    println!("app_state.info: {:?}", app_state.info.lock().unwrap());
    HttpResponse::Ok().body("Ok")
}
