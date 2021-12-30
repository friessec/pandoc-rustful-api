use crate::routes::v1::*;
use actix_web::web;

pub fn api_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(hello::hello)
            // .service(
            //     web::scope("/jobs")
            //         .service(
            //             web::resource("/{id}/process").route(web::post().to(job::process)),
            //         )
            // )
    );
}