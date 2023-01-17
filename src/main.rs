use std::fs::File;
use crate::api::KimaiApi;
mod api;

fn summary(kimai_api: KimaiApi, offset: i64) {
    let week_offset = chrono::Local::now() - chrono::Duration::weeks(offset);
    let sum = kimai_api.weekly(week_offset);
    println!("This week: {}h{}m{}s",
             sum.num_hours(),
             (sum.num_minutes()%60),
             (sum.num_seconds()%60)
    );
    println!("  Decimal: {:0.2}h",
             sum.num_hours() as f64 + ((sum.num_minutes()%60) as f64) /(60 as f64)
    );
}

fn main() {
    let f = File::open("config.yaml").expect("Could not open config file.");
    let api = KimaiApi::from_file(f);
    summary(api, 0)
}