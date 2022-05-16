use std::io::Read;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::operation::value::StringValueOperation;
use crate::core::main::string::pack::StringPack;
use crate::core::main::string::value::StringValue;
use crate::core::parse::from_deserializer;
use crate::core::traits::build::Buildable;
use crate::core::traits::operation::{ProvideOperation, ToOperation};
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::GeneralRequirements;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

impl GeneralCreation {
  fn next_requirements(&self, requirements: &GeneralRequirements) -> GeneralRequirements {
    let namespace = self.namespace.to_owned();
    let values = &self.values;
    requirements.next(namespace, values)
  }
}

impl<R> Buildable<StringValue, R> for GeneralCreation
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
  Operation(Vec<GeneralCreation>),
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
pub struct InnerGeneralCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
  elements: Vec<GeneralCreation>,
}