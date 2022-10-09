use std::process;
mod config;
mod action;
mod backend;
mod list;
mod new;
mod start;
mod error;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
  match config::read_config() {
    Ok(_) => {},
    Err(err) => {
      println!("{:?}", err);
      println!("Create '.zoom-cli' at your home directory.");
      process::exit(1);
    }
  }
  let parser = parser::ActionParser::new();
  let result = parser.action.run();
  result.print();
  Ok(())
}
