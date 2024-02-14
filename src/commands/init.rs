use crate::{
  config::{self, Config},
  constants,
  error::Error,
};
use clap::Args;
use inquire::Text;

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
  #[arg(short = 'y', long = "yes", long_help = "Answer yes to all prompts")]
  skip_prompt: bool,
}

pub fn execute(args: &InitArgs) {
  let config = config::get_config();
  if config.is_ok() {
    println!("This directory is already a shadow repository");
    return;
  }

  let config = match args.skip_prompt {
    true => get_default_config(),
    false => prompt_config(),
  };
  config::save_config(&config);
  println!("Shadow repository initialized");
}

fn get_default_config() -> Config {
  Config {
    fiction_dir: constants::DEFAULT_FICTION_DIR.to_string(),
  }
}

fn prompt_config() -> Config {
  let fiction_dir_result = Text::new("Enter the name of the fiction directory: ").prompt();
  let fiction_dir = match fiction_dir_result {
    Err(_) => Error::new("Failed to read fiction directory").print_and_exit(),
    Ok(fiction_dir) => {
      let fiction_dir = fiction_dir.trim();
      if fiction_dir.is_empty() {
        constants::DEFAULT_FICTION_DIR.to_string()
      } else {
        fiction_dir.to_string()
      }
    }
  };

  Config { fiction_dir }
}
