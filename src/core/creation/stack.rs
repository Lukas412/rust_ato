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
use crate::errors::build::creation::stack::CreationStackEmptyError;
use crate::errors::build::operation::OperationNotFoundError;

#[derive(Default)]
pub(crate) struct CreationStack {
  stack: Vec<Rc<Creation>>,
}

impl CreationStack {
  pub(crate) fn last(&self) -> error_stack::Result<Rc<Creation>, CreationStackEmptyError> {
    match self.stack.last() {
      Some(creation) => Ok(creation.clone()),
      None => Err(CreationStackEmptyError::new_report()),
    }
  }

  pub(crate) fn push(&mut self, creation: Rc<Creation>) {
    self.stack.push(creation)
  }

  pub(crate) fn pop(&mut self) -> error_stack::Result<(), CreationStackEmptyError> {
    match self.stack.pop() {
      Some(_) => Ok(()),
      None => Err(CreationStackEmptyError::new_report()),
    }
  }

  pub(crate) fn get_operation(&self, name: &ParameterName) -> error_stack::Result<Rc<Operation>, OperationNotFoundError> {
    let creation = self.get_creation(name.get_namespace())
      .change_context(OperationNotFoundError::new(name.get_name().clone()))?;
    creation.get_operation(name.get_name())
  }

  pub(crate) fn get_namespace(&self) -> error_stack::Result<&Namespace, CreationStackEmptyError> {
    let creation = self.last()?;
    Ok(creation.get_namespace())
  }

  pub(crate) fn build_on_stack(&mut self, creation: Rc<Creation>, pack_provider: &PackProvider) -> error_stack::Result<Value, BuildError> {
    self.push(creation);
    let creation = self.last().change_context(BuildError::default())?;
    let pack = pack_provider.get_pack(&creation.namespace).change_context(BuildError::default())?;
    let operation = pack.get_operation();
    let result = operation.build(pack_provider, self);
    self.pop().change_context(BuildError::default())?;
    result
  }

  fn get_creation(&self, namespace: &Namespace) -> error_stack::Result<Rc<Creation>, CreationNotFoundError> {
    let creation = self.stack.iter().filter(|creation| creation.get_namespace() == namespace).next();
    match creation {
      Some(creation) => Ok(creation.clone()),
      None => Err(CreationNotFoundError::new_report(namespace.clone()))
    }
  }
}