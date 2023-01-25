use std::fs::File;
use reqwest::Error;
use serde::Deserialize;
use chrono::{DateTime, Local, Duration};

#[derive(Deserialize, Debug)]
struct KimaiConfig {
    url: String,
    #[serde(default)]
    xauth: XAuth
}

#[derive(Deserialize, Debug)]
struct XAuth {
    pub user: String,
    pub token: String
}

impl Default for XAuth {
    fn default() -> Self {
        XAuth {
            user: String::from("no_user"),
            token: String::from("toomanysecrets")
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TimesheetEntry {
    id: u32,
    begin: DateTime<Local>,
    end: Option<DateTime<Local>>,
}

struct Timesheet {
    entries: Vec<TimesheetEntry>,
    page: u32,
    frozen: bool
}

impl Timesheet {
    pub fn freeze(&mut self) {
        self.frozen = true;
    }
    pub fn next_page(&mut self) -> u32 {
        self.page += 1;
        self.page
    }
}

pub struct KimaiApi {
    url: String,
    xauth: XAuth,
    timesheet: Timesheet
}

impl KimaiApi {
    pub fn from_file(fhandle: File) -> KimaiApi {
        let config: KimaiConfig = serde_yaml::from_reader(fhandle)
            .expect("Error loading config file.");
        KimaiApi {
            url: config.url,
            xauth: config.xauth,
            timesheet: Timesheet{
                entries: Vec::new(),
                page: 0,
                frozen: false
            }
        }
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

    fn get_timesheet_page(&self, page: u32) -> Result<Vec<TimesheetEntry>, reqwest::StatusCode> {
        let response = self.get(
            "/api/timesheets",
            Some(&format!("page={}", page)))
            .expect("HTTP-client error!");

        match response.status() {
            reqwest::StatusCode::OK => {
                let r: Vec<TimesheetEntry> = response.json()
                    .expect("Error decoding json.");
                Ok(r)
            }
            reqwest::StatusCode::NOT_FOUND => Err(reqwest::StatusCode::NOT_FOUND),
            status => panic!("Unexpected HTTP StatusCode: {}!", status),
        }
    }

    fn get_timesheet_entries(&mut self, earliest: DateTime<Local>) -> Result<&Vec<TimesheetEntry>, String>  {
        if !self.timesheet.frozen &&
           !self.timesheet.entries.iter().any(|e| e.begin < earliest ){
            loop {
                let page = self.timesheet.next_page();
                match self.get_timesheet_page(page) {
                    Ok(mut ts) => {
                        self.timesheet.entries.append(&mut ts);
                    }
                    Err(_e) => {
                        self.timesheet.freeze();
                        break;
                    }
                }
            }
        }
        Ok(&self.timesheet.entries)
    }

    pub fn summary(&mut self, start: DateTime<Local>, end: DateTime<Local>) -> Duration {
        let mut duration: Duration = Duration::minutes(0);
        let ts: &Vec<TimesheetEntry> = self.get_timesheet_entries(start).expect("Error retrieving timesheet entries");
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
