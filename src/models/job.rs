use serde::{Serialize, Deserialize};
use chrono::DateTime;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub creation_date: DateTime<chrono::Utc>,
}
