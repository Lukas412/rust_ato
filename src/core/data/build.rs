#[derive(Debug)]
pub enum BuildError {
  ValueError {
    value: String,
    backtrace: Backtrace,
  }
}

impl BuildError::ValueError {
  fn new(value: String) -> Self {
    Self {
      value,
      backtrace: Backtrace::default()
    }
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);