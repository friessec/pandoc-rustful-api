extern crate actix_web;
extern crate log;

mod routes;

use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG",
                      "actix_web=debug,actix_server=debug,pandoc_rustful_api=debug");
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap_api()
            .configure(routes::api::configuration)
            .with_json_spec_at("/swagger-ui/swagger.json")
            .build()
            // Mount swagger-ui after build
            .configure(routes::swagger::configuration)
    })
        .bind("127.0.0.1:8080")?;

    log::info!("Starting http server");

    server.run().await
}
