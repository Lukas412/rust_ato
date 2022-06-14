use std::fmt::Display;

use crate::Creation;
use crate::core::error::BuildError;
use crate::core::namespace::{Namespace, ParameterName};
use crate::core::operation::Operation;

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

  pub fn get_operation(&self, name: &ParameterName) -> Option<&Operation> {
    let creation = self.get_creation(name.get_namespace());
    match creation {
      Some(creation) => creation.get_operation(name.get_name()),
      None => None,
    }
  }

  pub fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_namespace();
    format!("at {} in {}", element, namespace)
  }

  pub fn get_namespace(&self) -> &Namespace {
    match self.stack.last() {
      Some(last) => last.get_namespace(),
      None => &Namespace::default(),
    }
  }

  pub fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}

impl CreationStack {
  fn get_creation(&self, namespace: &Namespace) -> Option<&Creation> {
    self.stack.iter()
      .filter(|creation| *creation.get_namespace() == *namespace)
      .next()
  }
}