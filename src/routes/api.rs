use crate::routes::v1::*;
use actix_web::web;

pub fn api_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/jobs")
                    .service(
                        web::resource("")
                            .route(web::get().to(jobs::job_list))
                            .route(web::post().to(jobs::job_create)),
                    ),
            )
    );
}