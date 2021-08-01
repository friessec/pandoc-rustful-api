use rocket::serde::json::Json;
//use rocket_contrib::uuid::Uuid;
//use uuid::Uuid;
use chrono::Utc;

use crate::models::job::Job;


#[openapi(tag = "Jobs")]
#[get("/jobs/<uuid>")]
pub fn get(uuid: String) -> Json<Job> {
    Json( Job {
        id: uuid,
        creationDate: Utc::now()
    })
}

#[openapi(tag = "Jobs")]
#[delete("/jobs/<uuid>")]
pub fn delete(uuid: String) -> Json<Job> {
    Json( Job {
        id: uuid,
        creationDate: Utc::now(),
    })
}
