use clap::Command;

mod app_error;
mod commands;
mod config;

fn main() {
  let init_command = commands::init::get_command();

  let matches = Command::new("shadow")
    .version("0.1.0")
    .about("A simple tool to manage the akashi shadow records")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(init_command)
    .get_matches();

  match matches.subcommand() {
    Some(("init", args)) => commands::init::run(args),
    _ => unreachable!(),
  };
}
