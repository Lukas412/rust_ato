use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;
use std::rc::Rc;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use value::GeneralCreationValue;

use crate::core::main::creation::stack::CreationStack;
use crate::core::main::error::BuildError;
use crate::core::main::general::operation::Operation;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::namespace::Namespace;
use crate::core::main::operation::Operation;
use crate::core::main::pack::provider::PackProvider;
use crate::core::main::string::value::StringValue;
use crate::core::parse::from_deserializer;
use crate::CreationStack;

pub mod value;
pub mod stack;

#[derive(Debug, Default)]
pub struct Creation {
  namespace: Namespace,
  operations: HashMap<String, Rc<Operation>>,
}

impl Creation {
  pub fn build(self, pack_provider: &PackProvider, requirements: &mut CreationStack) -> Result<StringValue, BuildError> {
    let pack = pack_provider.get_pack(&self.namespace)?;
    let operation = pack.operation();
    operation.build(pack_provider, requirements)
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
      HashMap::from_iter(inner.values.into_iter().map(GeneralCreationValue::to_name_and_operation));
    Creation { namespace, operations }
  }
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
struct InnerCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}