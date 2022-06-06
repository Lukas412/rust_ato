use std::fmt::Display;

use crate::{BuildError, GeneralCreation};
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::operation::ProvideOperationWithNamespace;

#[derive(Default)]
pub struct CreationStack {
  stack: Vec<GeneralCreation>,
}

impl CreationStack {
  pub fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", element, namespace)
  }
}

impl CreationStack {
  fn requirement_box(&self, namespace: &Namespace) -> Option<&GeneralCreation> {
    self.stack.iter()
      .filter(|requirement_box| requirement_box.get_namespace() == namespace)
      .next()
  }
}

impl CreationStack {
  pub fn get_namespace(&self) -> &Namespace {
    match self.stack.last() {
      Some(last) => last.get_namespace(),
      None => &self.namespace,
    }
  }

  pub fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}

impl ProvideOperationWithNamespace<StringOperation> for CreationStack {
  type Value = StringValue;
  fn operation(&self, namespace: &Namespace, name: &String) -> Result<StringOperation, BuildError> {
    match self.requirement_box(namespace) {
      Some(requirement_box) => requirement_box.operation(name),
      None => Err(BuildError::new_operation_not_found_error(name.to_owned(), namespace.to_owned())),
    }
  }
}