use crate::core::build::error::BuildError::{Pack, Requirement, Value};

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
  Pack {
    namespace: String,
    backtrace: Backtrace,
  }
}

impl BuildError {
  pub fn new_value(value: String, namespace: String) -> Self {
    Value { value, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_requirement(name: String, namespace: String) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_pack(namespace: String) -> Self {
    Pack { namespace, backtrace: Backtrace::default() }
  }
}

#[derive(Debug, Default)]
pub struct Backtrace(Vec<String>);