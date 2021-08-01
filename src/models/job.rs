use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub creationDate: DateTime<chrono::Utc>,
}
