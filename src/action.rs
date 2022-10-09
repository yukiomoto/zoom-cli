/// This trait represents a command or an action to get executed.
pub trait Action {
  fn run(&self) -> Box<dyn ActionResult>;
}

/// This trait represents an execution result of an action.
pub trait ActionResult {
  fn print(&self);
}

