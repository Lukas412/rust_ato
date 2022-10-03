use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::rc::Rc;

use error_stack::{bail, report, ResultExt};
use serde::Deserialize;

use crate::{CreationStack, PackProvider};
use crate::core::creation::value::CreationValue;
use crate::core::namespace::Namespace;
use crate::core::operation::Operation;
use crate::core::value::Value;
use crate::errors::attachments::NamespaceInformation;
use crate::errors::build::BuildError;
use crate::errors::build::operation::OperationNotFoundError;

pub(crate) mod value;
pub(crate) mod stack;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Creation {
  namespace: Namespace,
  operations: HashMap<String, Rc<Operation>>,
}

impl Creation {
  pub(crate) fn build(self: Rc<Self>, pack_provider: &PackProvider, stack: &mut CreationStack) -> error_stack::Result<Value, BuildError> {
    stack.build_on_stack(self, pack_provider)
  }

  pub(crate) fn get_operation(&self, name: &String) -> error_stack::Result<Rc<Operation>, OperationNotFoundError> {
    match self.operations.get(name) {
      Some(operation) => Ok(operation.clone()),
      None => Err(OperationNotFoundError::new_report(name.clone()))
        .attach_printable(NamespaceInformation::new(self.namespace.clone())),
    }
  }

  pub(crate) fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl Creation {
  fn from_inner(inner: InnerCreation) -> Creation {
    let namespace = Namespace::new(inner.namespace);
    let operations =
      HashMap::from_iter(inner.values.into_iter().map(CreationValue::to_name_and_operation));
    Creation { namespace, operations }
  }
}

#[derive(Debug, Default, Deserialize)]
struct InnerCreation {
  namespace: String,
  values: Vec<CreationValue>,
}
