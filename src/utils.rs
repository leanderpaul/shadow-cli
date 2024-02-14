use std::fs;

use crate::error::Error;

pub fn save_to_file<C: AsRef<[u8]>>(file_path: &String, contents: C) {
  let path_dirs = file_path.split("/").collect::<Vec<&str>>();
  if path_dirs.len() > 1 {
    let dirs = path_dirs[..path_dirs.len() - 1].join("/");
    let result = fs::create_dir_all(&dirs);
    if result.is_err() {
      let msg = format!("Failed to create the directory: {}", &dirs);
      Error::new(&msg).print_and_exit();
    }
  }

  let result = fs::write(file_path, contents);
  if result.is_err() {
    let msg = format!("Failed to write the file: {}", file_path);
    Error::new(&msg).print_and_exit();
  }
}
