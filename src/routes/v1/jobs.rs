use actix_web::{web, Result};
use serde::{Serialize, Deserialize};
use paperclip::actix::{
    Apiv2Schema,
    api_v2_operation,
    web::Json,
};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Job {
    name: String,
    id: Option<i64>,
}

#[api_v2_operation]
pub async fn job_list() -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: None,
    };
    Ok(web::Json(job))
}

#[api_v2_operation]
pub async fn job_create() -> Result<Json<Job>, ()> {
    let job = Job {
        name: "Test".to_string(),
        id: None,
    };
    Ok(web::Json(job))
}