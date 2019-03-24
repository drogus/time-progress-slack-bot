#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;

use rocket::State;
use rocket_contrib::json::{JsonValue};
use chrono::{Date, TimeZone, Utc};

const NUMBER_OF_CHARS_IN_PROGRESS_BAR: u8 = 20;

struct AConfig {
  start_date: Date<Utc>,
  end_date: Date<Utc>
}

#[post("/progress")]
fn progress(config: State<AConfig>) -> JsonValue {
    let end_date = config.end_date;
    let start_date = config.start_date;
    let today = Utc::today();

    let days_till_end = end_date.signed_duration_since(today).num_days();
    let all_days = end_date.signed_duration_since(start_date).num_days();
    let message = format!("{} days left untill {}", days_till_end, end_date.format("%A, %e %B %Y"));
    let progress: f32 = (all_days - days_till_end) as f32 / all_days as f32;
    let percent = (progress * 100_f32).round() as u8;
    let filled_chars = (NUMBER_OF_CHARS_IN_PROGRESS_BAR as f32 * progress).round() as u8;
    let rest_chars = NUMBER_OF_CHARS_IN_PROGRESS_BAR - filled_chars;
    let mut progressbar = String::new();
    for _i in 0..filled_chars {
        progressbar.push_str("▓");
    }
    for _i in 0..rest_chars {
        progressbar.push_str("░");
    }

    json!({
        "response_type": "in_channel",
        "text": message,
        "attachments": [
            {
                "text": format!("{} {}%", progressbar, percent)
            }
        ]
    })
}

fn read_date_from_env_var(var_name: String) -> Date<Utc> {
    let env_var_date = std::env::var(&var_name)
        .expect(&format!("{} env var is not set, exiting", var_name));
    let date_str = format!("{}T00:00:00", env_var_date);
    let datetime = Utc::datetime_from_str(&Utc, &date_str.to_owned(), "%Y-%m-%dT%H:%M:%S").
        expect(&format!("{} needs to be specified with %Y-%m-%d format, exiting", var_name));

    datetime.date()
}

fn main() {
    let config = AConfig {
        start_date: read_date_from_env_var("START_DATE".to_string()),
        end_date: read_date_from_env_var("END_DATE".to_string()),
    };
    rocket::ignite().mount("/", routes![progress]).manage(config).launch();
}
