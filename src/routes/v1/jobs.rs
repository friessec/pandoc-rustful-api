use std::io::Write;
use actix_multipart::Multipart;
use actix_web::Error;
use futures_util::TryStreamExt as _;
use paperclip::actix::{api_v2_operation, web, web::Json, web::Path};
use paperclip::actix::web::HttpResponse;
use uuid::Uuid;
use crate::DEFAULT_WORKDIR;

use crate::models::job::Job;
use crate::services::job_service;

#[api_v2_operation]
pub async fn job_list() -> Result<Json<Vec<Job>>, ()> {
    Ok(job_service::find_all())
}

#[api_v2_operation]
pub async fn job_create() -> Result<Json<Job>, ()> {
    let job = job_service::create();
    Ok(job)
}

#[api_v2_operation]
pub async fn job_get(path: Path<(uuid::Uuid, )>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_delete(path: Path<(uuid::Uuid, )>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_upload(path: Path<(uuid::Uuid, )>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let id = path.into_inner().0;
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |file| sanitize_filename::sanitize(file),
        );

        let filepath = format!("{}/{}/{}", DEFAULT_WORKDIR, id, filename);

        let mut file = web::block(|| std::fs::File::create(filepath)).await?;
        while let Some(chunk) = field.try_next().await? {
            file = web::block(move || file.write_all(&chunk).map(|_| file)).await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[api_v2_operation]
pub async fn job_process(path: Path<(uuid::Uuid, )>) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().into())
}