use std::process::Command;
use serde::Deserialize;
use crate::action;
use crate::error;
use crate::backend;

pub struct Start {
  pub id: i64,
}

impl action::Action for Start {
  fn run(&self) -> Box<dyn action::ActionResult> {
    match backend::single_meeting(self.id.to_string()) {
      Ok(res) => {
        match res.json::<Payload>() {
          Ok(payload) => {
            Box::new(StartActionResult {
              start_url: payload.start_url,
            })
          }
          Err(_) => {
            Box::new(error::ErrorActionResult {
              message: format!("{}", "Invalid meeting ID was specified."),
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

pub struct StartActionResult {
  start_url: String,
}

impl action::ActionResult for StartActionResult {
  fn print(&self) {
    // currently only support macos.
    Command::new("sh")
      .arg("-c")
      .arg(format!("open {}", self.start_url))
      .output()
      .expect("failed to execute process");
  }
}

#[derive(Debug, Deserialize)]
struct Payload {
  start_url: String,
}
