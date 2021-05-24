use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;
use rocket_okapi::swagger_ui::*;
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Job {
   pub id: Uuid,
}

#[openapi]
#[get("/jobs")]
pub fn jobs() -> Json<Vec<Job>> {
   let mut jobs = Vec::new();

   jobs.push(Job {
      id: Uuid::new_v4(),
   });

   jobs.push(Job {
      id: Uuid::new_v4(),
   });

   Json(jobs)
}

#[openapi]
#[post("/jobs")]
pub fn create() -> Json<Job> {
   Json( Job {
      id: Uuid::new_v4(),
   })
}