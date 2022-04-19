use std::fmt::Debug;
use std::io::Cursor;
use log::{error, info};
use reqwest::multipart;
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use anyhow::Result;

pub struct Client {
    api_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    pub id: Option<uuid::Uuid>,
}

impl Client {
    pub fn new(address: &str, port: usize) -> Self {
        let api = "api/v1";
        if port == 443 || port == 80 {
            return Client { api_address: format!("{}/{}", address, api) };
        }
        Client { api_address: format!("{}:{}/{}", address, port, api) }
    }

    pub async fn list(&self) -> Result<(), anyhow::Error> {
        let url = self.uri_builder("jobs");
        let res = self.call_get(url).await?;

        let jobs = res.json::<Vec<Job>>().await?;
        info!("\nJob List:");
        for job in jobs.iter() {
            info!("\t{}", job.id.unwrap());
        }
        Ok(())
    }

    pub async fn create(&self) -> Result<(), anyhow::Error> {
        let url = self.uri_builder("jobs");
        let client = reqwest::Client::builder()
            .build()?;

        let res = client.post(url).send().await?;
        if res.status().is_server_error() {
            error!("Response: {:?} {}", res.version(), res.status());
            error!("Headers: {:#?}\n", res.headers());
            return Ok(());
        }
        let job = res.json::<Job>().await?;
        info!("Created new job: {}", job.id.unwrap());
        Ok(())
    }

    pub async fn status(&self, id: &uuid::Uuid) -> Result<(), anyhow::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.call_get(url).await?;

        let job = res.json::<Job>().await?;
        info!("{:#?}", job);
        Ok(())
    }

    pub async fn delete(&self, id: &uuid::Uuid) -> Result<(), anyhow::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.call_delete(url).await?;

        let _job = res.text().await?;
        info!("Deleted job {}", id);
        Ok(())
    }

    pub async fn upload(&self, id: &uuid::Uuid, file: &str) -> Result<(), anyhow::Error> {
        let url = self.uri_builder(format!("jobs/{}/upload", id).as_str());

        let file = File::open(file).await?;
        let stream = FramedRead::new(file, BytesCodec::new());

        let stream = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(stream)
            .file_name("report.md");

        let form = multipart::Form::new()
             .part("file_data", part);

        let client = reqwest::Client::new();

        let res = client
            .post(url)
            .multipart(form)
            .send()
            .await?;

        if res.status().is_server_error() {
            error!("Response: {:?} {}", res.version(), res.status());
            error!("Headers: {:#?}\n", res.headers());
            return Ok(());
        }
        Ok(())
    }

    pub async fn process(&self, id: &uuid::Uuid) -> Result<(), anyhow::Error> {
        let url = self.uri_builder(format!("jobs/{}/process", id).as_str());
        let _res = self.call_get(url).await?;

        info!("Report generated");
        Ok(())
    }

    pub async fn download(&self, id: &uuid::Uuid, file: &str) -> Result<(), anyhow::Error> {
        let url = self.uri_builder(format!("jobs/{}/download", id).as_str());
        let res = self.call_get(url).await?;

        let mut file = std::fs::File::create(file)?;
        let mut content =  Cursor::new(res.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }

    ////////////////////////////
    // HELPER functions
    ////////////////////////////
    async fn call_get(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let res = reqwest::get(url).await?;
        if res.status().is_server_error() {
            error!("Response: {:?} {}", res.version(), res.status());
            error!("Headers: {:#?}\n", res.headers());
            return res.error_for_status();
        }
        Ok(res)
    }

    async fn call_delete(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::builder()
            .build()?;

        let res = client.delete(url).send().await?;
        if res.status().is_server_error() {
            error!("Response: {:?} {}", res.version(), res.status());
            error!("Headers: {:#?}\n", res.headers());
            return res.error_for_status();
        }
        Ok(res)
    }

    fn uri_builder(&self, route: &str) -> String {
        format!("{}/{}", self.api_address, route)
    }
}