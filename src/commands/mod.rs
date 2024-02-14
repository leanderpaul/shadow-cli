use clap::Subcommand;

use self::init::InitArgs;

pub mod init;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
  #[command(about = "Initialize a new shadow repository")]
  Init(InitArgs),
}
