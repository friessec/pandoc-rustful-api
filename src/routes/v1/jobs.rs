use paperclip::actix::{
    api_v2_operation,
    web::Json,
    web::Path,
};

use crate::models::job::Job;

#[api_v2_operation]
pub async fn job_list() -> Result<Json<Vec<Job>>, ()> {
    // TODO currently always the same folder is listed
    let uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6");
    let id = match uuid {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };
    let job = Job {
        id,
    };
    Ok(Json(vec!(job)))
}

#[api_v2_operation]
pub async fn job_create() -> Result<Json<Job>, ()> {
    // TODO currently always the same folder is used
    let uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6");
    let id = match uuid {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };
    let job = Job {
        id,
    };
    Ok(Json(job))
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