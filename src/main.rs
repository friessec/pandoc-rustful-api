#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

extern crate serde;
extern crate serde_json;

use rocket_okapi::swagger_ui::{SwaggerUIConfig, make_swagger_ui};

pub mod routes;

//use rocket::Data;
//use rocket::http::ContentType;
// #[openapi(skip)]
// #[post("/generate", data = "<file>")]
// fn post_generate(content_type: &ContentType, file: Data) -> String {
//     // let byt = file.open(10.megabytes());
//     content_type.to_string()
// }

fn main() {
    rocket::ignite()
        .mount(
            "/api/v1/",
            routes_with_openapi![
                //post_generate,
                routes::v1::pandoc::jobs::jobs,
            ]
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/v1/openapi.json".to_owned(),
                ..Default::default()
            })
        )
        .launch();

    // rocket::build()
    //      .mount(
    //          "/api/v1/",
    //          routes_with_openapi![post_generate]
    //      )
    //      .mount(
    //          "/swagger-ui/",
    //          make_swagger_ui(&get_docs())
    //      )
}