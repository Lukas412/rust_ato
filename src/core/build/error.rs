use crate::core::build::error::BuildError::{Requirement, Value};

pub mod value;
pub mod requirement;

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
  fn new_value(value: String, namespace: String) -> Self {
    Value { value, namespace, backtrace: Backtrace::default() }
  }

  fn new_requirement(name: String, namespace: String) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);