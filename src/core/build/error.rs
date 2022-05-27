use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use crate::core::build::error::BuildError::{Pack, Requirement, Value};

pub enum BuildError {
  Value {
    name: String,
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
  pub fn new_value(name: String, namespace: String) -> Self {
    Value { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_requirement(name: String, namespace: String) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_pack(namespace: String) -> Self {
    Pack { namespace, backtrace: Backtrace::default() }
  }
}

impl BuildError {
  fn message(&self) -> String {
    match self {
      Value { .. } => format!(""),
      Requirement { .. } => format!(""),
      Pack { .. } => format!(""),
    }
  }

  fn backtrace(&self) -> &Backtrace {
    match self {
      Value { backtrace, .. } => backtrace,
      Requirement { backtrace,.. } => backtrace,
      Pack { backtrace,.. } => backtrace,
    }
  }
}

impl Display for BuildError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let error_message = self.message();
    let backtrace = self.backtrace();
    write!(f, "{}{}", backtrace, error_message)
  }
}

#[derive(Default)]
pub struct Backtrace(Vec<String>);

impl Display for Backtrace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let backtrace = &self.0;
    let traces: String = backtrace
      .iter()
      .map(|trace| format!("{}\n", trace))
      .collect();
    write!(f, "{}", traces)
  }
}