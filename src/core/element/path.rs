use std::path::PathBuf;

mod operation;

struct PathElement {
  value: PathBuf,
}

impl PathElement {
  pub fn new(value: PathBuf) -> PathElement {
    PathElement { value }
  }
}