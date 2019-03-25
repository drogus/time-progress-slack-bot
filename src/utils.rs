extern crate chrono;
use chrono::{Date, TimeZone, Utc};

pub const NUMBER_OF_CHARS_IN_PROGRESS_BAR: u8 = 20;

pub struct DaysCount {
    pub all_days: i64,
    pub remaining_days: i64
}

pub fn read_date_from_env_var(var_name: &str) -> Date<Utc> {
    let env_var_date = std::env::var(&var_name)
        .expect(&format!("{} env var is not set, exiting", var_name));
    let date_str = format!("{}T00:00:00", env_var_date);
    let datetime = Utc::datetime_from_str(&Utc, &date_str.to_owned(), "%Y-%m-%dT%H:%M:%S").
        expect(&format!("{} needs to be specified with %Y-%m-%d format, exiting", var_name));

    datetime.date()
}

pub fn get_days_counts(start_date: Date<Utc>, end_date: Date<Utc>) -> DaysCount {
    let today = Utc::today();

    let remaining_days = end_date.signed_duration_since(today).num_days();
    let all_days = end_date.signed_duration_since(start_date).num_days();

    DaysCount { all_days: all_days, remaining_days: remaining_days }
}

pub fn calculate_progress(count: &DaysCount) -> f32 {
  (count.all_days - count.remaining_days) as f32 / count.all_days as f32
}

pub fn calculate_percent(count: &DaysCount) -> f32 {
    calculate_progress(&count) * 100_f32
}

pub fn generate_progressbar(count: &DaysCount) -> String {
    let progress = calculate_progress(&count);
    let filled_chars = (NUMBER_OF_CHARS_IN_PROGRESS_BAR as f32 * progress).round() as u8;
    let rest_chars = NUMBER_OF_CHARS_IN_PROGRESS_BAR - filled_chars;
    let mut progressbar = String::new();
    for _i in 0..filled_chars {
        progressbar.push_str("▓");
    }
    for _i in 0..rest_chars {
        progressbar.push_str("░");
    }
    return progressbar
}

