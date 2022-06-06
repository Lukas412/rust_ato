use std::fmt::{Display, Formatter};

use crate::BuildError::{OperationNotFound, UnknownXmlNamespace};
use crate::core::build::error::BuildError::{Pack, Requirement, Value};
use crate::core::traits::namespace::Namespace;

pub struct BuildError {
  message: String,
  namespace: Namespace,
  backtrace: Backtrace,
}

impl BuildError {
  pub fn new_operation_not_found_error(name: &String, namespace: Namespace) -> Self {
    let message = format!("OperationNotFound: {name}");
    Self::new(message, namespace)
  }

  pub fn new_unknown_xml_namespace_error(xml_namespace: &String, namespace: Namespace) -> Self {
    let message = format!("UnknownXmlNamespace: {xml_namespace}");
    Self::new(message, namespace)
  }

  pub fn new_can_not_convert_text_to_value_error(text: &String, namespace: Namespace) -> Self {
    let message = format!("CanNotConvertTextToValue: {text}");
    Self::new(message, namespace)
  }
}

impl BuildError {
  pub fn add_backtrace(&mut self, trace: String) {
    self.backtrace.add(trace)
  }
}

impl BuildError {
  fn new(message: String, namespace: Namespace) -> Self {
    let backtrace = Backtrace::default();
    Self { message, namespace, backtrace }
  }
}

impl Display for BuildError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let message = &self.message;
    let namespace = &self.namespace;
    let backtrace = &self.backtrace;
    write!(f, "{}{} in {}", backtrace, message, namespace)
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