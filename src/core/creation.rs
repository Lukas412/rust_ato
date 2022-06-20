use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use core::creation::value::CreationValue;
use core::error::BuildError;
use core::namespace::Namespace;
use core::operation::Operation;
use core::parse::from_deserializer;
use core::value::Value;

use crate::{CreationStack, PackProvider};

pub mod value;
pub mod stack;

#[derive(Debug, Default)]
pub struct Creation {
  namespace: Namespace,
  operations: HashMap<String, Operation>,
}

impl Creation {
  pub fn build(self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    stack.build_on_stack(self, pack_provider)
  }

  pub fn get_operation(&self, name: &String) -> Result<&Operation, BuildError> {
    match self.operations.get(name) {
      Some(operation) => Ok(operation),
      None => Err(BuildError::new_operation_not_found_error(name, self.namespace.to_owned())),
    }
  }

  pub fn get_owned_namespace(&self) -> Namespace {
    self.namespace.to_owned()
  }
}

impl YaDeserialize for Creation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerCreation = from_deserializer(reader)?;
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
