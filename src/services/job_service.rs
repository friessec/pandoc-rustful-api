use std::path::{Path, PathBuf};
use actix_web::web::Json;
use uuid::Uuid;
use crate::models::job::Job;

fn job_dir(id: &str, path: &str) -> PathBuf {
    let workdir = Path::new(path);
    let job_dir = workdir.join(id);

    job_dir
}

fn save_job(_id: Uuid, job: Job) -> Job {

    job
}

fn load_job(_id: Uuid) -> Job {
    let job = Job {
        id: None,
    };

    job
}

pub fn find_all() -> Json<Vec<Job>> {
    // TODO currently always the same folder is listed
    let uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6");
    let id = match uuid {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };
    let job = Job {
        id,
    };
    Json(vec!(job))
}

pub fn create(workdir: &str) -> Json<Job> {
    // TODO currently always the same folder is used
    let uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6");
    let id = match uuid {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };

    let path = job_dir(id.unwrap().to_string().as_str(), workdir);


    match std::fs::create_dir(path.clone()) {
        Ok(_) => log::debug!("Created new job directory: {}", path.to_str().unwrap()),
        Err(_) =>log::warn!("Job directory does already exist: {}", path.to_str().unwrap()),
    };

    let job = Job {
        id,
    };

    Json(save_job(uuid.unwrap(), job))
}

pub fn get(id: Uuid) -> Json<Job> {
    let job = load_job(id);

    Json(job)
}
