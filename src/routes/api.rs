use crate::routes::v1::*;
use paperclip::actix::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/jobs")
                    .service(
                        web::resource("")
                            .route(web::get().to(jobs::job_list))
                            .route(web::post().to(jobs::job_create)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(jobs::job_get))
                            .route(web::delete().to(jobs::job_delete))
                    )
                    .service(
                        web::resource("/{id}/upload")
                            .route(web::post().to(jobs::job_upload))
                    )
                    .service(
                        web::resource("/{id}/process")
                            .route(web::get().to(jobs::job_process))
                    )
            )
    );
}