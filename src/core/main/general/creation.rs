use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;
use std::rc::Rc;

use yaserde::YaDeserialize;
use yaserde::de::Deserializer;

use value::GeneralCreationValue;

use crate::{PackProvider, Requirements};
use crate::core::build::error::BuildError;
use crate::core::main::general::operation::GeneralOperation;
use crate::core::main::string::pack::StringPack;
use crate::core::main::string::value::StringValue;
use crate::core::parse::from_deserializer;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::core::traits::value::Value;

pub mod value;

#[derive(Debug, Default)]
pub struct GeneralCreation {
  namespace: Namespace,
  operations: HashMap<String, Rc<GeneralOperation>>,
}

impl GeneralCreation {
  fn from_inner(inner: InnerGeneralCreation) -> GeneralCreation {
    let namespace = inner.namespace;
    let operations =
      HashMap::from_iter(inner.values.into_iter().map(GeneralCreationValue::to_name_and_operation));
    GeneralCreation { namespace, operations }
  }
}

impl YaDeserialize for GeneralCreation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreation = from_deserializer(reader)?;
    Ok(Self::from_inner(inner))
  }
}

impl Buildable<StringValue> for InnerGeneralCreation {
  fn build(self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<StringValue, BuildError> {
    let pack: &StringPack = pack_provider.pack(&self.namespace)?;
    let operation = pack.operation();
    operation.build(pack_provider, requirements)
  }
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
struct InnerGeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}
