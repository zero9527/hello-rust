use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::actix::app_state::{AppState, Info};

#[get("/get/info")]
async fn handle_get(name: web::Query<String>, app_state: web::Data<AppState>) -> impl Responder {
    println!("data: {:?}", name);
    if let Some(i) = &app_state.get(&name) {
        return HttpResponse::Ok().body(format!("{:?}", i));
    }

    return HttpResponse::BadRequest().body(format!("name: {} 不存在", &name));
}

#[post("/add/info")]
async fn handle_add(req_body: web::Json<Info>, app_state: web::Data<AppState>) -> impl Responder {
    println!("data: {:?}", req_body);
    if let Some(_) = &app_state.get(&req_body.name) {
        return HttpResponse::BadRequest().body(format!("name: {} 已存在", &req_body.name));
    }

    if let Err(e) = &app_state.add(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[post("/update/info")]
async fn handle_update(
    req_body: web::Json<Info>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    println!("data: {:?}", req_body);
    if let None = &app_state.get(&req_body.name) {
        return HttpResponse::BadRequest().body(format!("name: {} 不存在", &req_body.name));
    }

    if let Err(e) = &app_state.update(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[delete("delete/info")]
async fn handle_delete(name: web::Query<String>, app_state: web::Data<AppState>) -> impl Responder {
    println!("data: {:?}", name);
    if let None = &app_state.get(&name) {
        return HttpResponse::BadRequest().body(format!("name: {} 不存在", &name));
    }

    if let Err(e) = &app_state.remove(name.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}
