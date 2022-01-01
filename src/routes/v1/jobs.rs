use paperclip::actix::{
    api_v2_operation,
    web::Json,
    web::Path,
};

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
pub async fn job_get(path: Path<(uuid::Uuid,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_delete(path: Path<(uuid::Uuid,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_upload(path: Path<(uuid::Uuid,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_process(path: Path<(uuid::Uuid,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}