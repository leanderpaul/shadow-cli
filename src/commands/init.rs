use crate::config::{self, Config};
use clap::{Arg, ArgAction, ArgMatches, Command};
use inquire::error::InquireError;
use inquire::Text;

pub fn get_command() -> Command {
  let skip_prompt_arg = Arg::new("skip-prompt")
    .short('y')
    .long("yes")
    .long_help("Answer yes to all prompts")
    .action(ArgAction::SetTrue);

  return Command::new("init")
    .about("Initialize a new shadow repository")
    .arg(skip_prompt_arg);
}

fn get_config(skip_prompt: bool) -> Config {
  let encryption_key = "12345678901234567890123456789012".to_string();
  if skip_prompt {
    return Config { encryption_key, fiction_dir: None };
  }

  let fiction_dir = get_fiction_dir();
  let config = Config { encryption_key, fiction_dir };
  return config;
}

pub fn run(args: &ArgMatches) {
  let config = config::get_config();
  if config.is_ok() {
    println!("This directory is already a shadow repository");
    return;
  }

  let skip_prompt = args.get_flag("skip-prompt");
  let config = get_config(skip_prompt);
  let _ = config::save_config(&config);
  println!("Shadow repository initialized");
}

fn get_fiction_dir() -> Option<String> {
  let fiction_dir: Result<String, InquireError> = Text::new("Enter the name of the fiction directory: ").prompt();
  return match fiction_dir {
    Err(_) => None,
    Ok(fiction_dir) => {
      let fiction_dir = fiction_dir.trim();
      if fiction_dir.is_empty() {
        None
      } else {
        Some(fiction_dir.to_string())
      }
    }
  };
}
