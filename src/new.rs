use serde::Deserialize;
use crate::action;
use crate::error;
use crate::backend;

pub struct New {
  pub title: String,
  pub start: Option<String>,
  pub duration: Option<String>
}

impl action::Action for New {
  fn run(&self) -> Box<dyn action::ActionResult> {
    match backend::create_meeting(self.title.clone(), self.start.clone(), self.duration.clone()) {
      Ok(res) => {
        match res.json::<Payload>() {
          Ok(payload) => {
            Box::new(NewActionResult {
              join_url: payload.join_url,
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

pub struct NewActionResult {
  join_url: String,
}

impl action::ActionResult for NewActionResult {
  fn print(&self) {
    println!("\x1b[32mSuccessfully created a meeting!\x1b[0m\n{}", self.join_url);
  }
}

#[derive(Debug, Deserialize)]
pub struct Payload {
  join_url: String,
}
