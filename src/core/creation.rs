use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::rc::Rc;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use core::creation::value::CreationValue;
use core::error::BuildError;
use core::namespace::Namespace;
use core::operation::Operation;
use core::value::Value;
use helpers::ser::from::{from_deserializer, from_file};

use crate::{CreationStack, PackProvider};

pub(crate) mod value;
pub(crate) mod stack;

#[derive(Debug, Default)]
pub(crate) struct Creation {
  namespace: Namespace,
  operations: HashMap<String, Rc<Operation>>,
}

impl Creation {
  pub(crate) fn from_file<P: AsRef<Path> + ?Sized + Display>(path: &P) -> io::Result<Self> {
    from_file(path)?
  }

  pub(crate) fn build(self: Rc<Self>, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    stack.build_on_stack(self, pack_provider)
  }

  pub(crate) fn get_operation(&self, name: &String) -> Result<&Rc<Operation>, BuildError> {
    match self.operations.get(name) {
      Some(operation) => Ok(operation),
      None => Err(BuildError::new_operation_not_found_error(name, self.namespace.to_owned())),
    }
  }

  pub(crate) fn get_owned_namespace(&self) -> Namespace {
    self.namespace.to_owned()
  }
}

impl YaDeserialize for Creation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner = from_deserializer(reader)?;
    Ok(Self::from_inner(inner))
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

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation")]
struct InnerCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<CreationValue>,
}
