use std::path::PathBuf;

struct PathElement {
  value: PathBuf,
}

impl PathElement {
  pub fn new(value: PathBuf) -> PathElement {
    PathElement { value }
  }
}