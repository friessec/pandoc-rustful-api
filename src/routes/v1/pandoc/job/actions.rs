use rocket_contrib::json::Json;
use rocket::Data;
use rocket::http::ContentType;

use crate::models::job::Job;
use rocket_okapi::request::OpenApiFromData;


#[openapi(tag = "Jobs")]
#[post("/jobs/<uuid>/upload",
       data = "<data>")]
pub fn upload(uuid: String,
              content_type: &ContentType,
              data: Data) -> String {
    // let byt = file.open(10.megabytes());
    content_type.to_string()
}

#[openapi(tag = "Jobs")]
#[post("/jobs/<uuid>/generate")]
pub fn generate(uuid: String) -> String {

    "".to_string()
}

#[openapi(tag = "Jobs")]
#[get("/jobs/<uuid>/progress")]
pub fn progress(uuid: String) -> String {

    "".to_string()
}
