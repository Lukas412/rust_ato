#[derive(Debug)]
pub enum BuildError {
  Value(ValueError),
  Requirement(RequirementError),
}

#[derive(Debug)]
pub struct ValueError {
  value: String,
  namespace: String,
  backtrace: Backtrace,
}

impl ValueError {
  pub fn new(value: String, namespace: String) -> BuildError {
    BuildError::Value(Self {
      value,
      namespace,
      backtrace: Backtrace::default(),
    })
  }
}

#[derive(Debug)]
pub struct RequirementError {
  name: String,
  namespace: String,
  backtrace: Backtrace,
}

impl RequirementError {
  pub fn new(name: String, namespace: String) -> BuildError {
    BuildError::Requirement(Self {
      name,
      namespace,
      backtrace: Backtrace::default(),
    })
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);