use rocket_contrib::json::Json;
use uuid::Uuid;
use crate::models::job::Job;



#[openapi]
#[get("/jobs")]
pub fn jobs() -> Json<Vec<Job>> {
   let mut jobs = Vec::new();

   jobs.push(Job {
      id: Uuid::new_v4().to_string(),
   });

   jobs.push(Job {
      id: Uuid::new_v4().to_string(),
   });

   Json(jobs)
}

#[openapi]
#[post("/jobs")]
pub fn create() -> Json<Job> {
   Json( Job {
      id: Uuid::new_v4().to_string(),
   })
}
