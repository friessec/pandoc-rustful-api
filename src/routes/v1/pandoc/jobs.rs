use rocket::response::content::Json;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Jobs {
   pub name: u8,
}


#[openapi]
#[get("/jobs")]
pub fn jobs() -> Json<Vec<Jobs>> {
   let mut jobs = Vec::new();

   jobs.push(Jobs{
      name: 1,
   });

   Json(jobs)
}