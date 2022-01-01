extern crate actix_web;
extern crate log;

mod configs;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;
use configs::constants;
use crate::constants::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG",
                      "actix_web=debug,actix_server=debug,pandoc_rustful_api=debug");
    std::env::set_var("WORKDIR",
                      DEFAULT_WORKDIR);
    env_logger::init();

    let bind_addr = format!("{}:{}", DEFAULT_INTERFACE, DEFAULT_PORT);

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
        .bind(bind_addr.clone())?;

    create_work_directory()?;
    log::info!("Starting http server");
    log::info!("Swagger-UI: http://{}/swagger-ui/", bind_addr);
    server.run().await
}

fn create_work_directory() -> std::io::Result<()> {
    let workdir = std::env::var("WORKDIR").unwrap_or(DEFAULT_WORKDIR.to_string());
    let dir = std::path::Path::new(workdir.as_str());
    if dir.exists() {
        return if dir.is_dir() {
            log::debug!("Using work directory: {}", dir.display());
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists,
                                    "Provided path exists but is not a directory!"))
        };
    }
    std::fs::create_dir(dir)?;

    log::debug!("Created work directory: {}", dir.display());
    Ok(())
}