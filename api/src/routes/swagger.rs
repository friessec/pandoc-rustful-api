use actix_files::Files;
use actix_web::web;


pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Files::new("/swagger-ui", "./static/swagger-ui/")
            .index_file("index.html")
    );
}