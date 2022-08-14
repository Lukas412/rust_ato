use std::fmt::Display;
use std::rc::Rc;
use error_stack::{report, ResultExt};

use crate::{Creation, PackProvider};
use crate::core::namespace::{Namespace, ParameterName};
use crate::core::operation::Operation;
use crate::core::value::Value;
use crate::errors::attachments::NamespaceInformation;
use crate::errors::build::BuildError;
use crate::errors::build::creation::CreationNotFoundError;
use crate::errors::build::operation::OperationNotFoundError;

#[derive(Default)]
pub(crate) struct CreationStack {
  stack: Vec<Rc<Creation>>,
}

impl CreationStack {
  pub(crate) fn last(&self) -> Result<Rc<Creation>, CreationNotFoundError> {
    match self.stack.last() {
      Some(creation) => Ok(creation.clone()),
      None => Err(CreationNotFoundError::new_report()),
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
  pub(crate) fn get_operation(&self, name: &ParameterName) -> error_stack::Result<Rc<Operation>, OperationNotFoundError> {
    let namespace = name.get_namespace();
    let creation = self.get_creation(namespace);
    match creation {
      Some(creation) => creation.get_operation(name.get_name()),
      None => Err(OperationNotFoundError::new_report(name.get_name().clone()))
        .attach_printable(NamespaceInformation::new(namespace.clone())),
    }
  }

  pub(crate) fn get_owned_namespace(&self) -> error_stack::Result<Namespace, BuildError> {
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