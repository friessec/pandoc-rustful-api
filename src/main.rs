extern crate actix_web;

mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(routes::api::api_configuration)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
