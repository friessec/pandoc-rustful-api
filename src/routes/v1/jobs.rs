use actix_web::{web, Responder, Result};

pub async fn job_list() -> Result<impl Responder> {
    Ok(web::Json(vec![{}]))
}

pub async fn job_create() -> Result<impl Responder> {
    Ok(web::Json(vec![{}]))
}