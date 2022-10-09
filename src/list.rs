use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use serde::Deserialize;
use crate::config;
use crate::action;
use crate::error;
use crate::backend;

pub struct List {

}

impl action::Action for List {
  fn run(&self) -> Box<dyn action::ActionResult> {
    match backend::list_meetings() {
      Ok(res) => {
        match res.json::<ListResponsePayload>() {
          Ok(payload) => {
            Box::new(ListActionResult {
              meetings: payload.meetings,
            })
          }
          Err(err) => {
            Box::new(error::ErrorActionResult {
              message: format!("{:?}", err),
            })
          }
        }
      }
      Err(err) => {
        Box::new(error::ErrorActionResult {
          message: format!("{:?}", err),
        })
      }
    }
  }
}

pub struct ListActionResult {
  meetings: Vec<Meeting>,
}

impl action::ActionResult for ListActionResult {
  fn print(&self) {
    if self.meetings.len() == 0 {
      println!("There is no meetings.");
      return
    }
    let config = match config::read_config() {
      Ok(c) => c,
      Err(_) => {
        panic!("Create '.zoom-cli' at your home directory.");
      }
    };
    let tz: Tz = config.timezone.parse().unwrap();
    let mut output: String = "ID\tName\tTime\tJoin Url\n".to_owned();
    for meeting in &self.meetings {
      match DateTime::parse_from_rfc3339(&meeting.start_time) {
        Ok(dt) => {
          let timezoned_datetime = dt.with_timezone(&tz);
          let end = timezoned_datetime + Duration::minutes(meeting.duration);
          output += &format!("{} {} {}〜{} {}\n", meeting.id, meeting.topic, timezoned_datetime.format("%F %H:%M %p"), end.format("%H:%M %p"), meeting.join_url);
        }
        Err(_) => {
          output += &format!("{} {} {}\n", meeting.id, meeting.topic, "[FailedToParse]");
        }
      };
    }
    println!("{}", output);
  }
}

#[derive(Debug, Deserialize)]
struct ListResponsePayload {
  meetings: Vec<Meeting>,
}

#[derive(Debug, Deserialize)]
pub struct Meeting {
  id: i64,
  topic: String,
  start_time: String,
  duration: i64,
  join_url: String,
}
