pub struct AppError {
  msg: String,
}

impl AppError {
  pub fn new(msg: &str) -> AppError {
    AppError { msg: msg.to_string() }
  }

  pub fn print_and_exit(&self) -> ! {
    println!("Error: {}", self.msg);
    std::process::exit(1);
  }
}
