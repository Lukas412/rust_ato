use std::path::PathBuf;

pub trait EndsWithStr {
  fn ends_with_str(&self, s: &str) -> bool;
}

impl EndsWithStr for PathBuf {
  fn ends_with_str(&self, s: &str) -> bool {
    match self.to_str() {
      Some(string) => string.ends_with(s),
      None => false,
    }
  }
}