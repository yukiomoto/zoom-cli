use dirs;
use std::fs;
use std::process;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub token: String,
  pub timezone: String,
}

// TODO: do some memoization or store the result globally so you dont have to read every time you need to reference.
pub fn read_config() -> Result<Config> {
  let json = match dirs::home_dir() {
    Some(mut p) => {
      p.push(".zoom-cli");
      fs::read_to_string(p.as_path())
    },
    None => {
      println!("Could not find your home directory.");
      process::exit(1);
    }
  };
  match json {
    Ok(j) => {
      let config = serde_json::from_str::<Config>(&j);
      match config {
        Ok(c) => {
          return Ok(c);
        },
        Err(err) => {
          println!("{:?}", err);
          println!("Your configuration file has an invalid format.");
          process::exit(1);
        }
      }
    },
    Err(err) => {
      println!("{:?}", err);
      println!("Configuration file not found.");
      process::exit(1);
    }
  }
}
