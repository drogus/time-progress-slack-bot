#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate chrono;

use rocket::State;
use rocket_contrib::json::{JsonValue};
use chrono::{Date, TimeZone, Utc};

struct AConfig {
  until_date: Date<Utc>
}

#[post("/progress")]
fn progress(config: State<AConfig>) -> JsonValue {
    let date = config.until_date;
    let today = Utc::today();
    let days = date.signed_duration_since(today).num_days();
    let message = format!("Days till {}: {}", date.format("%Y-%m-%d"), days);
    json!({
        "response_type": "in_channel",
        "text": message
    })
}

fn main() {
    let env_var_date = std::env::var("UNTIL_DATE")
        .expect("UNTIL_DATE env var is not set, exiting");
    let date_str = env_var_date + &"T00:00:00".to_owned();
    let datetime = Utc::datetime_from_str(&Utc, &date_str.to_owned(), "%Y-%m-%dT%H:%M:%S").
        expect("UNTIL_DATE needs to be specified with %Y-%m-%d format, exiting");
    let config = AConfig {
        until_date: datetime.date()
    };
    rocket::ignite().mount("/", routes![progress]).manage(config).launch();
}
