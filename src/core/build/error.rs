use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use crate::BuildError::OperationNotFound;
use crate::core::build::error::BuildError::{Pack, Requirement, Value};
use crate::core::traits::error::GetBacktrace;

pub enum BuildError {
  OperationNotFound {
    name: String,
    namespace: Option<String>,
    backtrace: Backtrace,
  },
  Pack {
    namespace: String,
    backtrace: Backtrace,
  },
  Requirement {
    name: String,
    namespace: String,
    backtrace: Backtrace,
  },
  Value {
    name: String,
    namespace: String,
    backtrace: Backtrace,
  },
}

impl BuildError {
  pub fn new_operation_not_found_error(name: String, namespace: Option<String>) -> Self {
    OperationNotFound { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_pack(namespace: String) -> Self {
    Pack { namespace, backtrace: Backtrace::default() }
  }

  pub fn new_requirement(name: String, namespace: String) -> Self {
    Requirement { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn new_value(name: String, namespace: String) -> Self {
    Value { name, namespace, backtrace: Backtrace::default() }
  }

  pub fn add_backtrace<T: GetBacktrace>(&mut self, from: &T) {
    let trace = from.backtrace();
    self.mut_backtrace().add(trace)
  }
}

impl BuildError {
  fn message(&self) -> String {
    match self {
      OperationNotFound { name, namespace, .. } => {
        match namespace {
          Some(namespace) => format!("OperationNotFound: '{} from {}'", name, namespace),
          None => format!("OperationNotFound: '{}", name),
        }
      },
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