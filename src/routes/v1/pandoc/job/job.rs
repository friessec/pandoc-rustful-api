use rocket_contrib::json::Json;
//use rocket_contrib::uuid::Uuid;
//use uuid::Uuid;

use crate::models::job::Job;

#[openapi]
#[get("/jobs/<uuid>")]
pub fn get(uuid: String) -> Json<Job> {
    Json( Job {
        id: uuid,
    })
}

#[openapi]
#[delete("/jobs/<uuid>")]
pub fn delete(uuid: String) -> Json<Job> {
    Json( Job {
        id: uuid,
    })
}
