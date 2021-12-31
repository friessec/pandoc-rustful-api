use serde::{Serialize, Deserialize};
use paperclip::actix::Apiv2Schema;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Job {
    pub id: Option<uuid::Uuid>,
}
