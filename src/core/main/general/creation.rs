use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::build::error::BuildError;
use crate::core::main::general::requirements::RequirementBox;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::operation::value::StringValueOperation;
use crate::core::main::string::pack::StringPack;
use crate::core::main::string::value::StringValue;
use crate::core::parse::from_deserializer;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::operation::{ProvideOperation, ToOperation};
use crate::core::traits::pack::{Pack, ProvidePack};

#[derive(Debug, Default)]
pub struct GeneralCreation {
  requirement: RequirementBox,
}

impl YaDeserialize for GeneralCreation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreation = from_deserializer(reader)?;
    Ok(Self { requirement: inner.to_requirement() })
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

impl InnerGeneralCreation {
  fn to_requirement(self) -> RequirementBox {
    let namespace = self.namespace;
    let operations = HashMap::from_iter(self.values.into_iter().map(GeneralCreationValue::to_name_and_operation));
    RequirementBox::new(namespace, operations)
  }
}

impl<R> BuildableWithRequirements<StringValue, R> for InnerGeneralCreation
  where R: ProvidePack<StringPack> + ProvideOperation<StringOperation>
{
  fn build(&self, requirements: &R) -> Result<StringValue, BuildError> {
    let next_requirements = requirements;
    let pack = next_requirements.pack()?;
    let operation = pack.operation();
    operation.build(next_requirements)
  }
}

#[derive(Debug)]
pub struct GeneralCreationValue {
  name: String,
  operation: GeneralCreationOperation,
}

impl GeneralCreationValue {
  fn new(name: String, operation: GeneralCreationOperation) -> Self {
    Self { name, operation }
  }

  pub fn to_name_and_operation(self) -> (String, GeneralCreationOperation) {
    (self.name, self.operation)
  }
}

#[derive(Debug)]
pub enum GeneralCreationOperation {
  Empty,
  Value(String),
  Operation(Vec<InnerGeneralCreation>),
}

impl ToOperation<StringOperation> for GeneralCreationOperation {
  fn to_operation(self) -> StringOperation {
    match self {
      Self::Empty => StringOperation::Empty,
      Self::Value(value) => StringValueOperation::new(value),
      Self::Operation(..) => todo!(),
    }
  }
}

impl YaDeserialize for GeneralCreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreationValue = from_deserializer(reader)?;
    let name = inner.name;
    let operation =
      match inner {
      InnerGeneralCreationValue { value: Some(value), .. } =>
        GeneralCreationOperation::Value(value),
      InnerGeneralCreationValue { elements, .. } if !elements.is_empty() =>
        GeneralCreationOperation::Operation(elements),
      InnerGeneralCreationValue { .. } =>
        GeneralCreationOperation::Empty
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
  elements: Vec<InnerGeneralCreation>,
}