extern crate redis;
use redis::Commands;

extern crate reqwest;

use std::collections::HashMap;

mod utils;
use self::utils::{read_date_from_env_var,get_days_counts,generate_progressbar,calculate_percent};

fn main() {
    let redis_url_str = std::env::var("REDIS_URL")
        .expect("REDIS_URL is not set");
    let redis_url = redis::parse_redis_url(&redis_url_str)
        .expect("Failed to parse REDIS_URL");
    let client = redis::Client::open(redis_url)
        .expect("Couldn't create Redis client");
    let conn = client.get_connection().expect("Couldn't open Redis connection");
    let last_value: u8 = conn.get("last_progress_percentage").unwrap_or(0);

    let start_date = read_date_from_env_var("START_DATE".to_string());
    let end_date = read_date_from_env_var("END_DATE".to_string());
    let count = get_days_counts(start_date, end_date);
    let percent: u8 = calculate_percent(&count).trunc() as u8;

    if percent > last_value {
        let result: redis::RedisResult<String> = conn.set("last_progress_percentage", percent);
        result.expect("Couldn't set the new percent value");
        let slack_webhook_url = std::env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK is not set");
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        let message = format!("Woohoo! {}% reached, {} days left! :dancing_avocados:", percent, count.remaining_days);
        println!("Sending message: {}", message);
        map.insert("text", message);

        let res = client.post(&slack_webhook_url.to_owned())
            .json(&map)
            .send()
            .expect("Couldn't send a Slack message");
    }
}
