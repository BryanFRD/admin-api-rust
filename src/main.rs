use std::sync::Arc;

use actix_cors::Cors;
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
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
                .send_wildcard())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::NormalizePath::default())
            .wrap(actix_web::middleware::Compress::default())
            .app_data(web::Data::new(docker.clone()))
            .route("/containers", web::get().to(list_containers))
    })
    .bind("127.0.0.1:8181")?
    .run()
    .await
}