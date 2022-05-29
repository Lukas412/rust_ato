use std::collections::HashMap;
use std::fmt::{Debug, Display};
use crate::BuildError;
use crate::core::main::general::operation::GeneralOperation;

use crate::core::main::string::operation::StringOperation;
use crate::core::traits::namespace::{default_namespace, GetNamespace, Namespace};
use crate::core::traits::operation::{GetOperation, ProvideOperation, ProvideOperationWithNamespace};

pub struct Requirements {
  namespace: Namespace,
  stack: Vec<RequirementBox>,
}

impl Requirements {
  pub fn new() -> Self {
    Self {
      namespace: default_namespace(),
      stack: Vec::default(),
    }
  }

  pub fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", element, namespace)
  }
}

impl Requirements {
  fn requirement_box(&self, namespace: &Namespace) -> Option<&RequirementBox> {
    self.stack.iter()
      .filter(|requirement_box| requirement_box.get_namespace() == namespace)
      .next()
  }
}

impl GetNamespace for Requirements {
  fn get_namespace(&self) -> &Namespace {
    match self.stack.last() {
      Some(last) => last.get_namespace(),
      None => &self.namespace,
    }
  }
}

impl ProvideOperationWithNamespace<StringOperation> for Requirements {
  fn operation(&self, namespace: &Namespace, name: &String) -> Result<StringOperation, BuildError> {
    match self.requirement_box(namespace) {
      Some(requirement_box) => requirement_box.operation(name),
      None => Err(BuildError::new_operation_not_found_error(name.to_owned(), namespace.to_owned())),
    }
  }
}

#[derive(Debug, Default)]
pub struct RequirementBox {
  namespace: Namespace,
  operations: HashMap<String, GeneralOperation>,
}

impl RequirementBox {
  pub fn new(namespace: Namespace, operations: HashMap<String, GeneralOperation>) -> Self {
    Self {
      namespace,
      operations,
    }
  }
}

impl GetNamespace for RequirementBox {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl ProvideOperation<GeneralOperation> for RequirementBox {
  fn operation(&self, name: &String) -> Result<&GeneralOperation, BuildError> {
    match self.operations.get(name) {
      Some(operation) => Ok(operation),
      None => Err(BuildError::new_operation_not_found_error(name.to_owned(), self.namespace.to_owned())),
    }
  }
}