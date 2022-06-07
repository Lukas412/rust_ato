use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;
use std::rc::Rc;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use value::CreationValue;

use crate::{CreationStack, PackProvider};
use crate::core::error::BuildError;
use crate::core::namespace::Namespace;
use crate::core::operation::Operation;
use crate::core::parse::from_deserializer;
use crate::core::value::Value;

pub mod value;
pub mod stack;

#[derive(Debug, Default)]
pub struct Creation {
  namespace: Namespace,
  operations: HashMap<String, Rc<Operation>>,
}

impl Creation {
  pub fn build(self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    let pack = pack_provider.get_pack(&self.namespace)?;
    let operation = pack.operation();
    operation.build(pack_provider, stack)
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
      HashMap::from_iter(inner.values.into_iter().map(CreationValue::to_name_and_rc_operation));
    Creation { namespace, operations }
  }
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
struct InnerCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<CreationValue>,
}
