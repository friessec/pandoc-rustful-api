use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::*;
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Jobs {
   pub name: String,
}

#[openapi]
#[get("/jobs")]
pub fn jobs() -> Json<Vec<Jobs>> {
   let mut jobs = Vec::new();

   jobs.push(Jobs{
      name: "Job1".to_string(),
   });

   jobs.push(Jobs{
      name: "Job2".to_string(),
   });

   Json(jobs)
}