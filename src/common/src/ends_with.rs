use std::path::PathBuf;

trait EndsWith {
  fn ends_with(&self, s: &str) -> bool;
}

impl EndsWith for PathBuf {
  fn ends_with(&self, s: &str) -> bool {
    match self.to_str() {
      Some(string) => string.ends_with(s),
      None => false,
    }
  }
}