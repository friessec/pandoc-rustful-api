use std::io::Write;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{Error, error};
use futures_util::TryStreamExt as _;
use paperclip::actix::{api_v2_operation, web, web::Json, web::Path};
use paperclip::actix::web::HttpResponse;
use uuid::Uuid;

use crate::AppSettings;
use crate::models::job::Job;
use crate::services::job_service;

#[api_v2_operation]
pub async fn job_list()
    -> Result<Json<Vec<Job>>, Error> {
    Ok(job_service::find_all())
}

#[api_v2_operation]
pub async fn job_create(config: web::Data<AppSettings>)
                        -> Result<Json<Job>, Error> {
    let job = job_service::create(config.pandoc.workdir.as_str());
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_get(path: Path<(uuid::Uuid, )>,
                     config: web::Data<AppSettings>)
                     -> Result<Json<Job>, Error> {
    let (id, ) = path.into_inner();
    let job = job_service::get(config.pandoc.workdir.as_str(), id);
    Ok(Json(job))
}

#[api_v2_operation]
pub async fn job_delete(path: Path<(uuid::Uuid, )>,
                        config: web::Data<AppSettings>)
                        -> Result<HttpResponse, Error> {
    let (id, ) = path.into_inner();
    let mut filepath = std::path::PathBuf::from(&config.pandoc.workdir);
    filepath.push(id.to_string());

    log::debug!("Remove job directory: {}", filepath.to_str().unwrap());
    match std::fs::remove_dir_all(filepath) {
        Ok(_) => (),
        Err(_) => return Err(error::ErrorNotFound("Could not delete this job"))
    };
    Ok(HttpResponse::Ok().into())
}

#[api_v2_operation]
pub async fn job_upload(path: Path<(uuid::Uuid, )>,
                        config: web::Data<AppSettings>, mut payload: Multipart)
                        -> Result<HttpResponse, Error> {
    let (id, ) = path.into_inner();
    while let Some(mut field) = payload.try_next().await? {
        let mut filepath = std::path::PathBuf::from(&config.pandoc.workdir);
        filepath.push(id.to_string());
        filepath.push("upload");

        if field.name() == "file_data" {
            let content_disposition = field.content_disposition();
            let filename = content_disposition.get_filename().map_or_else(
                || Uuid::new_v4().to_string(),
                sanitize_filename::sanitize,
            );
            filepath.push(filename);

            let mut file = web::block(|| std::fs::File::create(filepath)).await??;
            while let Some(chunk) = field.try_next().await? {
                file = web::block(move || file.write_all(&chunk).map(|_| file)).await??;
            }
        }
        else if field.name() == "zip_data" {
            log::info!("Received zip file");
            // TODO uncompress data
        }
        else {
            log::warn!("Unknown data uploaded! Metadata {}", field.name())
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[api_v2_operation]
pub async fn job_process(path: Path<(uuid::Uuid, )>,
                         config: web::Data<AppSettings>)
                         -> Result<HttpResponse, Error> {
    let (id, ) = path.into_inner();
    let mut filepath = std::path::PathBuf::from(&config.pandoc.workdir);
    filepath.push(id.to_string());

    let pandoc_cmd = format!(r#"pandoc {} \
        -o {}.pdf\
        --from markdown+yaml_metadata_block+raw_html \
        --template eisvogel.tex \
        --table-of-contents \
        --toc-depth 6 \
        --number-sections \
        --top-level-division=chapter \
        --highlight-style breezedark"#,
                             "upload/report.md",
                             config.pandoc.file_output_name
    );

    log::debug!("In workdir: {}", filepath.display());
    log::debug!("Run: {}", pandoc_cmd);

    let output = web::block(move || std::process::Command::new("sh")
        .arg("-c")
        .arg(pandoc_cmd)
        .current_dir(filepath)
        .output()).await??;
    if !output.status.success() {
        log::error!("Pandoc error: {}", String::from_utf8(output.stderr).unwrap());
        return Err(error::ErrorInternalServerError("Could not process files"));
    }

    Ok(HttpResponse::Ok().into())
}

#[api_v2_operation]
pub async fn job_download(path: Path<(uuid::Uuid, )>,
                          config: web::Data<AppSettings>)
                          -> Result<actix_files::NamedFile, Error> {
    let (id, ) = path.into_inner();
    let mut filepath = std::path::PathBuf::from(&config.pandoc.workdir);
    filepath.push(id.to_string());
    filepath.push(config.pandoc.file_output_name.to_string() + ".pdf");

    Ok(NamedFile::open(filepath)?)
}