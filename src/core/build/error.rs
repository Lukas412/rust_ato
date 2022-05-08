use crate::core::build::error::BuildError::{Requirement, Value};

#[derive(Debug)]
pub enum BuildError {
  Value {
    value: String,
    namespace: String,
    backtrace: Backtrace,
  },
  Requirement {
    name: String,
    namespace: String,
    backtrace: Backtrace,
  },
}

impl BuildError {
  pub(crate) fn new_value(value: String, namespace: String) -> Self {
    Value { value, namespace, backtrace: Backtrace::default() }
  }

  pub(crate) fn new_requirement(name: String, namespace: String) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);