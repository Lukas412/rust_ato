use std::fmt::Display;

use crate::{BuildError, GeneralCreation};
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::operation::ProvideOperationWithNamespace;

#[derive(Default)]
pub struct GeneralCreationStack {
  stack: Vec<GeneralCreation>,
}

impl GeneralCreationStack {
  pub fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", element, namespace)
  }
}

impl GeneralCreationStack {
  fn requirement_box(&self, namespace: &Namespace) -> Option<&GeneralCreation> {
    self.stack.iter()
      .filter(|requirement_box| requirement_box.get_namespace() == namespace)
      .next()
  }
}

impl GetNamespace for GeneralCreationStack {
  fn get_namespace(&self) -> &Namespace {
    match self.stack.last() {
      Some(last) => last.get_namespace(),
      None => &self.namespace,
    }
  }
}

impl ProvideOperationWithNamespace<StringOperation> for GeneralCreationStack {
  type Value = StringValue;
  fn operation(&self, namespace: &Namespace, name: &String) -> Result<StringOperation, BuildError> {
    match self.requirement_box(namespace) {
      Some(requirement_box) => requirement_box.operation(name),
      None => Err(BuildError::new_operation_not_found_error(name.to_owned(), namespace.to_owned())),
    }
  }
}