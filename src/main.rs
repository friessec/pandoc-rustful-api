extern crate actix_web;

mod routes;

use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .configure(routes::api::configuration)
            .with_json_spec_at("/swagger-ui/swagger.json")
            .build()
            // Mount swagger-ui after build
            .configure(routes::swagger::configuration)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
