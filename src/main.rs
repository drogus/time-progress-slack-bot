#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;

use rocket::State;
use rocket_contrib::json::{JsonValue};
use chrono::{Date, Utc};

mod utils;
use self::utils::{read_date_from_env_var,get_days_counts,generate_progressbar,calculate_percent};

struct AConfig {
  start_date: Date<Utc>,
  end_date: Date<Utc>
}

#[post("/progress")]
fn progress(config: State<AConfig>) -> JsonValue {
    let end_date = config.end_date;
    let start_date = config.start_date;

    let count = get_days_counts(start_date, end_date);

    let message = format!("{} days left untill {}", count.remaining_days, end_date.format("%A, %e %B %Y"));
    let progressbar = generate_progressbar(&count);

    json!({
        "response_type": "in_channel",
        "text": message,
        "attachments": [
            {
                "text": format!("{} {:.1}%", progressbar, calculate_percent(&count))
            }
        ]
    })
}

fn main() {
    let config = AConfig {
        start_date: read_date_from_env_var("START_DATE"),
        end_date: read_date_from_env_var("END_DATE"),
    };
    rocket::ignite().mount("/", routes![progress]).manage(config).launch();
}
