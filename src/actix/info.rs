use std::sync::Mutex;

use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::actix::app_state::{AppState, Info};

#[get("/get/info/{name}")]
async fn handle_get(
    name: web::Path<String>,
    app_state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("data: {:?}", name);
    println!("app_state.info: {:?}", app_state.lock().unwrap().info);
    if let Some(i) = app_state.lock().unwrap().get(&name) {
        return HttpResponse::Ok().body(format!("{:?}", i));
    }

    return HttpResponse::BadRequest().body(format!("name: {} 不存在", &name));
}

#[post("/add/info")]
async fn handle_add(
    req_body: web::Json<Info>,
    app_state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("[/add/info] req_body: {:?}", req_body);
    if let Some(_) = app_state.lock().unwrap().get(&req_body.name) {
        return HttpResponse::BadRequest().body(format!("name: {} 已存在", &req_body.name));
    }

    if let Err(e) = app_state.lock().unwrap().add(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[post("/update/info")]
async fn handle_update(
    req_body: web::Json<Info>,
    app_state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("req_body: {:?}", req_body);
    if let None = app_state.lock().unwrap().get(&req_body.name) {
        return HttpResponse::BadRequest().body(format!("name: {} 不存在", &req_body.name));
    }

    if let Err(e) = app_state.lock().unwrap().update(req_body.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    HttpResponse::Ok().body("Ok")
}

#[delete("/delete/{name}")]
async fn handle_delete(
    name: web::Path<String>,
    app_state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("name: {:?}", name);
    println!("app_state.info: {:?}", app_state.lock().unwrap().info);
    if let None = app_state.lock().unwrap().get(&name) {
        return HttpResponse::BadRequest().body(format!("name: {} 不存在", &name));
    }

    if let Err(e) = app_state.lock().unwrap().remove(name.into_inner()) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    println!("app_state.info: {:?}", app_state.lock().unwrap().info);
    HttpResponse::Ok().body("Ok")
}
