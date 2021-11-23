#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

extern crate serde;
extern crate serde_json;

use rocket_okapi::swagger_ui::{SwaggerUIConfig, make_swagger_ui};
use rocket::{Rocket, Build};

pub mod routes;
pub mod models;

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api/v1/",
            openapi_get_routes![
                routes::v1::pandoc::jobs::jobs,
                routes::v1::pandoc::jobs::create,
                routes::v1::pandoc::job::job::get,
                routes::v1::pandoc::job::job::delete,
                routes::v1::pandoc::job::actions::upload,
                routes::v1::pandoc::job::actions::generate,
                routes::v1::pandoc::job::actions::progress,
            ]
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/v1/openapi.json".to_owned(),
                ..Default::default()
            })
        )
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket()
        .launch()
        .await {
        eprintln!("Failed  to launch rocket! Port already used?");
        drop(e);
    }
}