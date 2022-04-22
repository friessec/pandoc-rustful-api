use actix_web;
use actix_cors::Cors;
use log;

mod configs;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;

use actix_web::{App, HttpServer, web};
//use actix_web_httpauth::middleware::HttpAuthentication;
use paperclip::actix::OpenApiExt;
use configs::constants;
use crate::configs::settings::AppSettings;
use crate::constants::*;
//use crate::middlewares::authentication::validate_token;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG",
                      "actix_web=debug,actix_server=debug,pandoc_rustful_api=debug");
    std::env::set_var("WORKDIR",
                      DEFAULT_WORKDIR);
    let settings = AppSettings::new();
    env_logger::init();

    create_work_directory(settings.clone())?;

    let bind_addr = format!("{}:{}", DEFAULT_INTERFACE, DEFAULT_PORT);
    let server = HttpServer::new(move || {
        //let auth = HttpAuthentication::bearer(validate_token);
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(settings.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
            //.wrap(auth)
            .wrap_api()
            .configure(routes::api::configuration)
            .with_json_spec_at("/swagger-ui/swagger.json")
            .build()
            // Mount swagger-ui after build
            .configure(routes::swagger::configuration)
    })
        .bind(bind_addr.clone())?;

    log::info!("Starting http server");
    log::info!("Swagger-UI: http://{}/swagger-ui/", bind_addr);
    server.run().await
}

fn create_work_directory(settings: AppSettings) -> std::io::Result<()> {
    let workdir = settings.pandoc.workdir;
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