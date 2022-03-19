use serde::{Serialize, Deserialize};

pub struct Client {
    api_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    pub id: Option<uuid::Uuid>,
}

impl Client {
    pub fn new(address: String, port: usize) -> Self {
        let api = "api/v1";
        if port == 443 || port == 80 {
            return Client { api_address: format!("{}/{}", address, api) };
        }
        Client { api_address: format!("{}:{}/{}", address, port, api) }
    }

    pub async fn list(&self) -> Result<(), reqwest::Error> {
        let url = self.uri_builder("jobs");
        let res = self.get(url).await?;

        let jobs = res.json::<Vec<Job>>().await?;
        println!("\nJob List:");
        for job in jobs.iter() {
            println!("\t{}", job.id.unwrap());
        }
        Ok(())
    }

    pub async fn create(&self) -> Result<(), reqwest::Error> {
        let url = self.uri_builder("jobs");
        let client = reqwest::Client::builder()
            .build()?;

        let res = client.post(url).send().await?;
        if res.status().is_server_error() {
            eprintln!("Response: {:?} {}", res.version(), res.status());
            eprintln!("Headers: {:#?}\n", res.headers());
            return Ok(());
        }
        let job = res.json::<Job>().await?;
        println!("Created new job: {}", job.id.unwrap());
        Ok(())
    }

    pub async fn status(&self, id: &uuid::Uuid) -> Result<(), reqwest::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.get(url).await?;

        let job = res.json::<Job>().await?;
        println!("{:#?}", job);
        Ok(())
    }

    pub async fn delete(&self, id: &uuid::Uuid) -> Result<(), reqwest::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.get(url).await?;

        let job = res.json::<Job>().await?;
        println!("{:#?}", job);
        Ok(())
    }

    pub async fn upload(&self, id: &uuid::Uuid, file: &str) -> Result<(), reqwest::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.get(url).await?;

        let job = res.json::<Job>().await?;
        println!("{:#?}", job);
        Ok(())
    }

    pub async fn process(&self, id: &uuid::Uuid) -> Result<(), reqwest::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.get(url).await?;

        let job = res.json::<Job>().await?;
        println!("{:#?}", job);
        Ok(())
    }

    pub async fn download(&self, id: &uuid::Uuid, file: &str) -> Result<(), reqwest::Error> {
        let url = self.uri_builder(format!("jobs/{}", id).as_str());
        let res = self.get(url).await?;

        let job = res.json::<Job>().await?;
        println!("{:#?}", job);
        Ok(())
    }

    ////////////////////////////
    // HELPER functions
    ////////////////////////////
    async fn get(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let res = reqwest::get(url).await?;
        if res.status().is_server_error() {
            eprintln!("Response: {:?} {}", res.version(), res.status());
            eprintln!("Headers: {:#?}\n", res.headers());
            return res.error_for_status();
        }
        Ok(res)
    }

    fn uri_builder(&self, route: &str) -> String {
        format!("{}/{}", self.api_address, route)
    }
}