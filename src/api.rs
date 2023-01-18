use std::fs::File;
use reqwest::Error;
use serde::Deserialize;
use chrono::{DateTime, Local, Duration};

#[derive(Deserialize, Debug)]
pub struct KimaiApi {
    url: String,
    xauth: XAuth,
    max_pages: Option<u32>
}

#[derive(Deserialize, Debug)]
struct XAuth {
    pub user: String,
    pub token: String
}

#[derive(Deserialize, Debug)]
pub struct Timesheet {
    id: u32,
    begin: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

impl KimaiApi {
    pub fn from_file(fhandle: File) -> KimaiApi {
        serde_yaml::from_reader(fhandle).expect("Error loading config file.")
    }

    fn get(&self, path: &str, parameters: Option<&str>) -> Result<reqwest::blocking::Response, Error> {
        let url = match parameters {
            Some(p) => format!("{}{}?{}", self.url, path, p),
            None => format!("{}{}", self.url, path)
        };
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .header("X-AUTH-USER", &self.xauth.user)
            .header("X-AUTH-TOKEN", &self.xauth.token)
            .send()?;
        Ok(response)
    }

    fn get_entries(&self) -> Result<Vec<Timesheet>, Error>  {
        let mut ts: Vec<Timesheet> = Vec::new();
        for i in 1..=10 {
            let response = self.get("/api/timesheets", Some(&format!("page={}", i)))?;
            match response.status() {
                reqwest::StatusCode::NOT_FOUND => break,
                reqwest::StatusCode::OK => {
                    let mut _ts: Vec<Timesheet> = response.json()?;
                    ts.append(&mut _ts);
                }
                status => panic!("Unknown HTTP StatusCode: {}", status),
            }
        }
        Ok(ts)
    }

    pub fn summary(&self, start: DateTime<Local>, end: DateTime<Local>) -> Duration {
        let mut duration: Duration = Duration::minutes(0);
        let ts: Vec<Timesheet> = self.get_entries().expect("Error retrieving timesheet entries");
        for entry in ts.iter() {
            if entry.begin > start && entry.begin < end {
                duration = duration + match entry.end {
                    Some(end) => end - entry.begin,
                    None => Local::now() - entry.begin
                };
            }
        }
        duration
    }
}
