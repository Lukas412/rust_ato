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
  pub fn new(value: &str, namespace: &String) -> BuildError {
    BuildError::Value(Self {
      value: value.to_owned(),
      namespace: namespace.to_owned(),
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
  pub fn new(name: &String, namespace: &String) -> BuildError {
    BuildError::Requirement(Self {
      name: name.to_owned(),
      namespace: namespace.to_owned(),
      backtrace: Backtrace::default(),
    })
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);