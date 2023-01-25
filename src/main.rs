use std::fs::File;
use chrono::{Datelike, DateTime, Local};
use now::{DateTimeNow, WeekStartDay};
use clap::{Parser, Subcommand};
use crate::api::KimaiApi;
mod api;

#[derive(Parser)]
#[command(about = "kimaic-rs - prints time entries from kimai")]
#[command(author, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Week {
        offset: Option<i64>,
    },
    Month { },
}

fn today(kimai_api: &mut KimaiApi) {
    let now = chrono::Local::now();
    let sod = now.beginning_of_day();
    let eod = now.end_of_day();
    let daily = kimai_api.summary(sod, eod);
    println!("Today:\t{}h", daily.num_minutes() as f64 / 60.0);
}

fn weekly(kimai_api: &mut KimaiApi, now: DateTime<Local>) {
    let sow = now.beginning_of_week_with_start_day(&WeekStartDay::Monday);
    let eow = now.end_of_week();
    let weekly = kimai_api.summary(sow, eow);
    print!("Week #{:2} [{} - {}]:\t",
             sow.iso_week().week(), sow.format("%Y-%m-%d"), eow.format("%Y-%m-%d"),
    );
    print!("{}:{} ", weekly.num_hours(), weekly.num_minutes() % 60);
    println!("({:.2}h)", weekly.num_minutes() as f64 / 60.0);
}

fn load_kimai_api() -> KimaiApi {
    let f = File::open("config.yaml").expect("Could not open config file.");
    KimaiApi::from_file(f)
}

fn main() {
    let cli = Cli::parse();
    let mut api = load_kimai_api();
    match &cli.command {
        Some(Commands::Week { offset }) => {
            let offset = match offset {
                Some(offset) => *offset,
                None => 0
            };
            weekly(&mut api, chrono::Local::now() - chrono::Duration::weeks(offset));
            if offset == 0 {
                today(&mut api);
            }
        }
        Some(Commands::Month { }) => {
            let now = chrono::Local::now();
            let mut cur = now.beginning_of_month();
            while cur < now {
                weekly(&mut api, cur);
                cur = cur.beginning_of_week() + chrono::Duration::weeks(1);
            }
        }
        None => {}
    }
}