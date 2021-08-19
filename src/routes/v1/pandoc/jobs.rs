use rocket::serde::json::Json;
use uuid::Uuid;
use chrono::Utc;

use crate::models::job::Job;


#[openapi(tag = "Jobs")]
#[get("/jobs")]
pub fn jobs() -> Json<Vec<Job>> {
   let mut jobs = Vec::new();

   jobs.push(Job {
      id: Uuid::new_v4().to_string(),
      creation_date: Utc::now(),
   });

   jobs.push(Job {
      id: Uuid::new_v4().to_string(),
      creation_date: Utc::now(),
   });

   Json(jobs)
}

#[openapi(tag = "Jobs")]
#[post("/jobs")]
pub fn create() -> Json<Job> {
   Json( Job {
      id: Uuid::new_v4().to_string(),
      creation_date: Utc::now(),
   })
}
