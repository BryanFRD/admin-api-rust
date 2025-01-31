use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bollard::Docker;

async fn list_containers(docker: web::Data<Arc<Docker>>) -> impl Responder {
    match docker.list_containers::<String>(None).await {
        Ok(containers) => HttpResponse::Ok().json(containers),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error listing containers: {}", e)),   
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let docker = Arc::new(docker);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(docker.clone()))
            .route("/containers", web::get().to(list_containers))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}