use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};

pub fn from_current_path<P: AsRef<Path>>(path: &P) -> io::Result<PathBuf> {
  Ok(current_dir()?.join(path))
}