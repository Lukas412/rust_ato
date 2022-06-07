use std::fmt::Display;

use crate::{BuildError, Creation};
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::operation::ProvideOperationWithNamespace;

#[derive(Default)]
pub struct CreationStack {
  stack: Vec<Creation>,
}

impl CreationStack {
  pub fn last(&self) -> Result<&Creation, BuildError> {
    match self.stack.last() {
      Some(creation) => Ok(creation),
      None => Err(BuildError::new_creation_stack_empty_error()),
    }
  }

  pub fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", element, namespace)
  }
}

impl CreationStack {
  fn requirement_box(&self, namespace: &Namespace) -> Option<&Creation> {
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