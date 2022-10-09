use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use crate::config;

pub fn list_meetings() -> Result<reqwest::blocking::Response, reqwest::Error> {
  let default_type = "upcoming";
  let client = reqwest::blocking::Client::new();
  client.get(endpoint("/users/me/meetings"))
    .header(reqwest::header::AUTHORIZATION, authorization_header())
    .query(&[("type", default_type)])
    .send()
}

pub fn single_meeting(id: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
  let client = reqwest::blocking::Client::new();
  client.get(endpoint(&format!("/meetings/{}", id)))
    .header(reqwest::header::AUTHORIZATION, authorization_header())
    .send()
}

/// Create a meeting through zoom api.
/// See https://marketplace.zoom.us/docs/api-reference/zoom-api/methods/#operation/meetingCreate
pub fn create_meeting(title: String, start: Option<String>, duration: Option<String>) -> Result<reqwest::blocking::Response, reqwest::Error> {
  fn format_start_time(start: &Option<String>) -> Option<String> {
    let start = match start {
      Some(start) => {
        start
      }
      None => {
        return None
      }
    };
    let config = match config::read_config() {
      Ok(c) => c,
      Err(_) => {
        panic!("Create '.zoom-cli' at your home directory.");
      }
    };
    let tz: Tz = config.timezone.parse().unwrap();
    let new_format = "%Y-%m-%dT%H:%M:%SZ";
    match NaiveDateTime::parse_from_str(&format!("{}:00", start), "%Y-%m-%d %H:%M:%S") {
      Ok(dt) => {
        let timezoned_datetime = tz.from_local_datetime(&dt).unwrap();
        return Some(timezoned_datetime.with_timezone(&Utc).format(new_format).to_string())
      }
      Err(_) => {
      }
    };
    match NaiveDateTime::parse_from_str(&format!("{}:00", start), "%Y/%m/%d %H:%M:%S") {
      Ok(dt) => {
        let timezoned_datetime = tz.from_local_datetime(&dt).unwrap();
        return Some(timezoned_datetime.with_timezone(&Utc).format(new_format).to_string())
      }
      Err(_) => {
      }
    };
    match DateTime::parse_from_rfc3339(start) {
      Ok(dt) => {
        return Some(dt.format(new_format).to_string())
      }
      Err(_) => {
      }
    };
    match DateTime::parse_from_rfc2822(start) {
      Ok(dt) => {
        return Some(dt.format(new_format).to_string())
      }
      Err(_) => {
      }
    };
    return None
  }

  #[derive(Serialize)]
  struct Payload {
    topic: String,
    start_time: Option<String>,
    duration: Option<String>,
  }
  let start_time = format_start_time(&start);
  let client = reqwest::blocking::Client::new();
  client.post(endpoint("/users/me/meetings"))
    .json::<Payload>(&Payload {
      topic: title,
      start_time: start_time,
      duration: duration,
    })
  .header(reqwest::header::AUTHORIZATION, authorization_header())
    .send()

}

fn endpoint(path: &str) -> String {
  format!("https://api.zoom.us/v2{}", path)
}
fn authorization_header() -> String {
  let config = match config::read_config() {
    Ok(c) => c,
    Err(_) => {
      panic!("Create '.zoom-cli' at your home directory.");
    }
  };
  format!("Bearer {}", config.token)
}
