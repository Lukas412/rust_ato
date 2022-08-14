use std::fmt::Display;
use std::rc::Rc;

use crate::{Creation, PackProvider};
use crate::core::namespace::{Namespace, ParameterName};
use crate::core::operation::Operation;
use crate::core::value::Value;

#[derive(Default)]
pub(crate) struct CreationStack {
  stack: Vec<Rc<Creation>>,
}

impl CreationStack {
  pub(crate) fn last(&self) -> Result<&Rc<Creation>, BuildError> {
    match self.stack.last() {
      Some(creation) => Ok(creation),
      None => Err(BuildError::new_creation_stack_empty_error()),
    }
  }

  pub(crate) fn push(&mut self, creation: Rc<Creation>) {
    self.stack.push(creation)
  }

  pub(crate) fn pop(&mut self) -> Result<(), BuildError> {
    match self.stack.pop() {
      Some(_) => Ok(()),
      None => Err(BuildError::new_creation_stack_empty_error()),
    }
  }
}

impl CreationStack {
  pub(crate) fn get_operation(&self, name: &ParameterName) -> Result<&Rc<Operation>, BuildError> {
    let namespace = name.get_namespace();
    let creation = self.get_creation(namespace);
    match creation {
      Some(creation) => creation.get_operation(name.get_name()),
      None => Err(BuildError::new_operation_not_found_error(name.get_name(), namespace.to_owned())),
    }
  }

  pub(crate) fn backtrace<T: Display>(&self, element: T) -> String {
    let namespace = self.get_owned_namespace();
    format!("at {element} in {namespace}")
  }

  pub(crate) fn get_owned_namespace(&self) -> Namespace {
    let creation = self.stack.last();
    match creation {
      Some(last) => last.get_owned_namespace(),
      None => Namespace::default(),
    }
  }

  pub(crate) fn build_on_stack(&mut self, creation: Rc<Creation>, pack_provider: &PackProvider) -> Result<Value, BuildError> {
    self.push(creation);
    let creation = self.last()?;
    let pack = pack_provider.get_pack(&creation.namespace)?;
    let operation = pack.get_operation();
    let result = operation.build(pack_provider, self);
    self.pop()?;
    result
  }
}

impl CreationStack {
  fn get_creation(&self, namespace: &Namespace) -> Option<&Creation> {
    for creation in self.stack.iter() {
      if &creation.get_owned_namespace() == namespace {
        return Some(creation)
      }
    }
    None
  }
}