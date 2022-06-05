use std::fmt::{Display, Formatter};
use crate::BuildError::OperationNotFound;
use crate::core::build::error::BuildError::{Pack, Requirement, Value};
use crate::core::traits::namespace::Namespace;

pub enum BuildError {
  OperationNotFound {
    name: String,
    namespace: Namespace,
    backtrace: Backtrace,
  },
  Pack {
    namespace: Namespace,
    backtrace: Backtrace,
  },
  Requirement {
    name: String,
    namespace: Namespace,
    backtrace: Backtrace,
  },
  Value {
    name: String,
    namespace: Namespace,
    backtrace: Backtrace,
  },
}

impl BuildError {
  pub fn new_operation_not_found_error(name: String, namespace: Namespace) -> Self {
    OperationNotFound { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_pack(namespace: Namespace) -> Self {
    Pack { namespace, backtrace: Backtrace::default() }
  }

  pub fn new_requirement(name: String, namespace: Namespace) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_value(name: String, namespace: Namespace) -> Self {
    Value { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn add_backtrace(&mut self, trace: String) {
    self.mut_backtrace().add(trace)
  }
}

impl BuildError {
  fn message(&self) -> String {
    match self {
      OperationNotFound { name, namespace, .. } => format!("OperationNotFound: '{} -> {}'", namespace, name),
      Value { .. } => format!(""),
      Requirement { .. } => format!(""),
      Pack { .. } => format!(""),
    }
  }

  fn backtrace(&self) -> &Backtrace {
    match self {
      OperationNotFound { backtrace, .. } => backtrace,
      Value { backtrace, .. } => backtrace,
      Requirement { backtrace,.. } => backtrace,
      Pack { backtrace,.. } => backtrace,
    }
  }

   fn mut_backtrace(&mut self) -> &mut Backtrace {
    match self {
      OperationNotFound { backtrace, .. } => backtrace,
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

impl Backtrace {
  fn add(&mut self, trace: String) {
    self.0.push(trace)
  }
}

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