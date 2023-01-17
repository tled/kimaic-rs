mod config;
mod api;

fn summary(config: &config::Config, offset: i64) {
    let week_offset = chrono::Local::now() - chrono::Duration::weeks(offset);
    let sum = api::summary(config, week_offset);
    println!("This week: {}h{}m{}s", sum.num_hours(), (sum.num_minutes()%60), (sum.num_seconds()%60));
    println!("  Decimal: {:0.2}h",
             sum.num_hours() as f64 + ((sum.num_minutes()%60) as f64) /(60 as f64)
    );
}

fn main() {
    let api_config = config::load_cfg().expect("Error loading config file!");
    summary(&api_config, 0);
}