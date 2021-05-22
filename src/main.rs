#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket_okapi::swagger_ui::*;
use rocket::Data;
use rocket::http::ContentType;


#[openapi(skip)]
#[post("/generate", data = "<file>")]
fn post_generate(content_type: &ContentType, file: Data) -> String {
    // let byt = file.open(10.megabytes());
    content_type.to_string()
}

fn main() {
    rocket::ignite()
        .mount(
            "/api/v1/",
            routes_with_openapi![
                post_generate
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