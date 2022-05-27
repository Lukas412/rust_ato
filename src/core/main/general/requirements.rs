use std::collections::HashMap;

use crate::core::main::general::creation::GeneralCreationOperation;
use crate::core::main::string::operation::StringOperation;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::operation::{GetOperation, ProvideOperation, ProvideOperationWithNamespace};

pub struct Requirements {
  namespace: Namespace,
  stack: Vec<RequirementBox>,
}

impl Requirements {
  pub fn new() -> Self {
    Self {
      namespace: Namespace::default(),
      stack: Vec::default(),
    }
  }

  pub fn backtrace(&self, name: &str) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", name, namespace)
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
  fn operation(&self, namespace: &Namespace, name: &String) -> Option<StringOperation> {
    self.requirement_box(namespace)?.operation(name)
  }
}

#[derive(Debug, Default)]
pub struct RequirementBox {
  namespace: Namespace,
  operations: HashMap<String, GeneralCreationOperation>,
}

impl RequirementBox {
  pub fn new(namespace: Namespace, operations: HashMap<String, GeneralCreationOperation>) -> Self {
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

impl ProvideOperation<StringOperation> for RequirementBox {
  fn operation(&self, name: &String) -> Option<StringOperation> {
    let general_operation = self.operations.get(name)?;
    Some(general_operation.get_operation())
  }
}