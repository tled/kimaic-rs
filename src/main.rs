use std::fs::File;
use chrono::Datelike;
use now::{DateTimeNow, WeekStartDay};
use crate::api::KimaiApi;
mod api;

fn summary(kimai_api: KimaiApi, offset: i64) {
    let now = chrono::Local::now() - chrono::Duration::weeks(offset);
    let sow = now.beginning_of_week_with_start_day(&WeekStartDay::Monday);
    let eow = now.end_of_week();
    let weekly = kimai_api.summary(sow, eow);
    if offset == 0 {
        let sod = now.beginning_of_day();
        let eod = now.end_of_day();
        let daily = kimai_api.summary(sod, eod);
        println!("Today: {}", daily.num_minutes() as f64 / 60.0);
    }

    println!("Week #{} [{} - {}]",
             sow.iso_week().week(), sow.format("%Y-%m-%d"), eow.format("%Y-%m-%d"),
    );
    println!("{}:{}", weekly.num_hours(), weekly.num_minutes() % 60);
    println!("{:.2}h", weekly.num_minutes() as f64 / 60.0)
}

fn main() {
    let f = File::open("config.yaml").expect("Could not open config file.");
    let api = KimaiApi::from_file(f);
    summary(api, 0)
}