use serde::{Serialize, Deserialize};
use paperclip::actix::{
    Apiv2Schema,
    api_v2_operation,
    web::Json,
    web::Path,
};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Job {
    name: String,
    id: Option<u32>,
}

#[api_v2_operation]
pub async fn job_list() -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: None,
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_create() -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: None,
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_get(path: Path<(u32,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_delete(path: Path<(u32,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_upload(path: Path<(u32,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_process(path: Path<(u32,)>) -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: Option::from(path.into_inner().0),
    };
    Ok(Json(job))
}