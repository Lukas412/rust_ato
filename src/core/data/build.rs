#[derive(Debug)]
pub enum BuildError {
  ValueError(ValueError)
}

#[derive(Debug)]
pub struct ValueError {
  value: String,
  backtrace: Backtrace,
}

impl ValueError {
  pub fn new(value: &str) -> BuildError {
    BuildError::ValueError(Self {
      value: value.to_string(),
      backtrace: Backtrace::default(),
    })
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);