use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use actix_web::web::Json;
use uuid::Uuid;
use crate::models::job::Job;

fn job_dir(id: &str, path: &str) -> PathBuf {
    let workdir = Path::new(path);
    let job_dir = workdir.join(id);

    job_dir
}

fn err_job() -> Job {
    Job {
        id: None,
    }
}

fn save_job(workdir: &str, id: Uuid, job: Job) -> Job {
    let mut filepath = std::path::PathBuf::from(workdir);
    filepath.push(id.to_string());
    filepath.push("job.json");
    let file = match File::create(filepath) {
        Ok(file) => file,
        Err(_) => return err_job()
    };
    let writer = BufWriter::new( file);

    match serde_json::to_writer_pretty(writer, &job) {
        Ok(_) => (),
        Err(_) => return err_job()
    };

    job
}

fn load_job(workdir: &str, id: Uuid) -> Job {
    let mut filepath = std::path::PathBuf::from(workdir);
    filepath.push(id.to_string());
    filepath.push("job.json");

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => return err_job()
    };
    let reader = BufReader::new( file);

    let job = match serde_json::from_reader(reader) {
        Ok(job) => job,
        Err(_) => err_job()
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

pub fn create(workdir: &str) -> Job {
    // TODO currently always the same folder is used
    let uuid = uuid::Uuid::parse_str("3fa85f64-5717-4562-b3fc-2c963f66afa6");
    let id = match uuid {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };

    let path = job_dir(id.unwrap().to_string().as_str(), workdir);

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => log::debug!("Created new job directory: {}", path.to_str().unwrap()),
        Err(_) =>log::warn!("Job directory does already exist: {}", path.to_str().unwrap()),
    };

    let upload_dir = path.join("upload");
    match std::fs::create_dir_all(upload_dir.clone()) {
        Ok(_) => log::debug!("Created upload directory: {}", upload_dir.to_str().unwrap()),
        Err(_) =>log::warn!("Upload directory does already exist: {}", upload_dir.to_str().unwrap()),
    };

    let job = Job {
        id,
    };

    save_job(workdir, uuid.unwrap(), job)
}

pub fn get(workdir: &str, id: Uuid) -> Job {
    let job = load_job(workdir, id);

    job
}
