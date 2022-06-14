use std::fmt::Display;

use crate::{Creation, PackProvider};
use crate::core::error::BuildError;
use crate::core::namespace::{Namespace, ParameterName};
use crate::core::operation::Operation;
use crate::core::value::Value;

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

  pub fn push(&mut self, creation: Creation) {
    self.stack.push(creation)
  }

  pub fn pop(&mut self) -> Result<(), BuildError> {
    match self.stack.pop() {
      Some(_) => Ok(()),
      None => Err(BuildError::new_creation_stack_empty_error()),
    }
  }
}

impl CreationStack {
  pub fn get_operation(&self, name: &ParameterName) -> Result<&Operation, BuildError> {
    let namespace = name.get_namespace();
    let creation = self.get_creation(namespace);
    match creation {
      Some(creation) => creation.get_operation(name.get_name()),
      None => Err(BuildError::new_operation_not_found_error(name.get_name(), namespace.to_owned())),
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

  pub fn build_on_stack(&mut self, creation: Creation, pack_provider: &PackProvider) -> Result<Value, BuildError> {
    self.push(creation);
    let result = self.last()?.build_on_stack(pack_provider, self);
    self.pop()?;
    result
  }
}

impl CreationStack {
  fn get_creation(&self, namespace: &Namespace) -> Option<&Creation> {
    for creation in self.stack.iter() {
      if creation.get_namespace() == namespace {
        return Some(creation)
      }
    }
    None
  }
}