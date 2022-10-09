use crate::action;

pub struct Error {
  pub message: String
}
impl action::Action for Error {
  fn run(&self) -> Box<dyn action::ActionResult> {
    Box::new(ErrorActionResult {
      message: self.message.clone(),
    })
  }
}
pub struct ErrorActionResult {
  pub message: String
}

impl action::ActionResult for ErrorActionResult {
  fn print(&self) {
    println!("\x1b[31mError: {}\x1b[0m", self.message)
  }
}
