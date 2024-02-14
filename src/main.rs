use crate::commands::Commands;
use clap::Parser;
use debug_print::debug_println;

mod commands;
mod config;
mod constants;
mod error;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "shadow", author, version)]
#[command(about = "A simple tool to manage the akashi shadow records")]
#[command(arg_required_else_help = true)]
#[command(subcommand_required = true)]
struct Cli {
  #[command(subcommand)]
  cmd: Commands,
}

fn main() {
  let cli = Cli::parse();
  debug_println!("cli: {:?}", cli);

  match &cli.cmd {
    Commands::Init(args) => commands::init::execute(args),
  }
}
