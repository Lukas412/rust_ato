use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::{PackProvider, Requirements};
use crate::core::build::error::BuildError;
use crate::core::main::general::operation::GeneralOperation;
use crate::core::main::general::requirements::RequirementBox;
use crate::core::main::string::pack::StringPack;
use crate::core::main::string::value::StringValue;
use crate::core::parse::from_deserializer;
use crate::core::traits::build::{BuildableWithRequirements};
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::core::traits::value::Value;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

impl GeneralCreation {
  fn get_operation<'a, P, O: Operation<V>, V>(&self, pack_provider: &'a PackProvider) -> Result<&'a O, BuildError>
    where PackProvider: ProvidePack<P, V>, P: 'a + Pack<V, Operation=O>, V: Value
  {
    let pack: &P = pack_provider.pack(&self.namespace)?;
    Ok(pack.operation())
  }
}

impl BuildableWithRequirements<StringValue> for GeneralCreation {
  fn to_requirement_box(self) -> RequirementBox {
    let namespace = self.namespace;
    let operations =
      HashMap::from_iter(self.values.into_iter().map(GeneralCreationValue::to_name_and_operation));
    RequirementBox::new(namespace, operations)
  }

  fn build(self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<StringValue, BuildError> {
    let pack: &StringPack = pack_provider.pack(&self.namespace)?;
    let operation = pack.operation();
    operation.build(pack_provider, requirements)
  }
}

#[derive(Debug)]
pub struct GeneralCreationValue {
  name: String,
  operation: GeneralOperation,
}

impl GeneralCreationValue {
  fn new(name: String, operation: GeneralOperation) -> Self {
    Self { name, operation }
  }

  pub fn to_name_and_operation(self) -> (String, GeneralOperation) {
    (self.name, self.operation)
  }
}

impl YaDeserialize for GeneralCreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreationValue = from_deserializer(reader)?;
    let (name, operation) =
      match inner {
        InnerGeneralCreationValue { name, value: Some(value), .. } =>
          (name, GeneralOperation::Value(value)),
        InnerGeneralCreationValue { name, elements, .. } if !elements.is_empty() =>
          (name, GeneralOperation::Operation(elements)),
        InnerGeneralCreationValue { name, .. } =>
          (name, GeneralOperation::Empty)
      };
    Ok(Self::new(name, operation))
  }
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
struct InnerGeneralCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
  elements: Vec<GeneralCreation>,
}