use std::process;

pub struct Error {
  msg: String,
}

impl Error {
  pub fn new(msg: &str) -> Error {
    Error { msg: msg.to_string() }
  }

  pub fn print_and_exit(&self) -> ! {
    println!("Error: {}", self.msg);
    process::exit(1);
  }
}
