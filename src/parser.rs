
use clap::{arg, Command};

use crate::action;
use crate::error;
use crate::list::List;
use crate::new::New;
use crate::start::Start;

/// This struct defines commands and their arguments
/// as well as which action each command should use.
///
/// See also [`action::Action`].
pub struct ActionParser {
  pub action: Box<dyn action::Action>
}

impl ActionParser {
  pub fn new() -> ActionParser {
    let matches = ActionParser::cli().get_matches();
    let action: Box<dyn action::Action> = match matches.subcommand() {
      Some(("ls", _sub_matches)) => {
        Box::new(List {})
      }
      Some(("new", sub_matches)) => {
        Box::new(New {
            title: sub_matches.get_one::<String>("TITLE").expect("required").to_owned(),
            start: match sub_matches.get_one::<String>("start") {
                Some(str) => {
                    Some(str.to_owned())
                }
                None => {
                    None
                }
            },
            duration: match sub_matches.get_one::<String>("duration") {
                Some(str) => {
                    Some(str.to_owned())
                }
                None => {
                    None
                }
            }
        })
      }
      Some(("start", sub_matches)) => {
        match sub_matches.get_one::<String>("ID").expect("required").parse::<i64>() {
          Ok(id) => {
            Box::new(Start {
              id: id,
            })
          }
          Err(_) => {
            Box::new(error::Error {
              message: format!("{}", "meeting ID must be an integer."),
            })
          }
        }
      }
      Some((command, _)) => {
        Box::new(error::Error {
          message: format!("`{}` is not suppored.", command),
        })
      }
      _ => {
        unreachable!();
      }
    };
    ActionParser { action }
  }

  fn cli() -> Command {
    Command::new("zoom")
      .about("command line interface for zoom")
      .arg_required_else_help(true)
      .allow_external_subcommands(true)
      .subcommand(
        Command::new("ls")
        .about("List upcoming meetings.")
        )
      .subcommand(
        Command::new("new")
        .about("Create a new meeting.")
        .arg(arg!(<TITLE> "Title of the meeting"))
        .arg(arg!(-s --start <START> "When the meeting starts"))
        .arg(arg!(-d --duration <DURATION> "How long is the meeting in minutes."))
        .arg_required_else_help(true),
        )
      .subcommand(
        Command::new("start")
        .about("Start an existing meeting.")
        .arg(arg!(<ID> "Meeting id to start"))
        .arg_required_else_help(true),
        )
  }
}

