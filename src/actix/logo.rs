use std::path::Path;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, HttpResponse, Responder};

use crate::actix::app_state::ResData;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    icon_type: Text<u8>,
    #[multipart(limit = "100KB")]
    data: TempFile,
}

#[post("/update/logo")]
async fn handle_update(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let icon_type_str = match form.icon_type.into_inner() {
        1 => "login_icon".to_string(),
        2 => "top_icon".to_string(),
        3 => "small_icon".to_string(),
        _ => "".to_string(),
    };
    println!("\n[/post/logo] icon_type: {:?}", icon_type_str);
    if let Some(name) = form.data.file_name {
        // 路径拼接
        let path = Path::new("./upload").join(Path::new(&name));
        println!("path: {:?}", path);
        // 保存文件
        form.data.file.persist(path).unwrap();
    }
    HttpResponse::Ok().json(ResData { code: 0, message: "Ok".to_string() })
}
