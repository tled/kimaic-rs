use reqwest::Error;
use serde::Deserialize;
use chrono::{DateTime, Local, Duration, Datelike};
use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct Timesheet {
    id: u32,
    begin: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

fn get(config: &Config, path: &str, parameters: Option<&str>) -> Result<reqwest::blocking::Response, Error> {
    let url = match parameters {
        Some(p) => format!("{}{}?{}", config.url, path, p),
        None => format!("{}{}", config.url, path)
    };
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("X-AUTH-USER", &config.auth.user)
        .header("X-AUTH-TOKEN", &config.auth.token)
        .send()?;
    Ok(response)
}

fn get_entries(config: &Config) -> Result<Vec<Timesheet>, Error>  {
    let mut ts: Vec<Timesheet> = Vec::new();
    for i in 1..=10 {
        let response = get(config, "/api/timesheets", Some(&format!("page={}", i)))?;
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

fn summary_sum(ts: Vec<Timesheet>, t: DateTime<Local>) -> Duration {
    let mut duration: Duration = Duration::minutes(0);
    for entry in ts.iter() {
        if entry.begin.iso_week().week() == t.iso_week().week() {
            duration = duration + match entry.end {
                Some(end) => end - entry.begin,
                None => Local::now() - entry.begin
            };
        }
    }
    duration
}

pub fn summary(config: &Config, t: DateTime<Local>) -> Duration {
    let ts: Vec<Timesheet> = get_entries(config).expect("Error retrieving timesheet entries");
    summary_sum(ts, t)
}
