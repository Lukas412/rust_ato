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
use crate::core::traits::creation::CreationValue;
use crate::core::traits::namespace::{Namespace, GetNamespace};
use crate::core::traits::pack::{Pack, ProvidePack};

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

impl GetNamespace for GeneralCreation {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl<R> Buildable<StringValue, R> for GeneralCreation
  where R: ProvidePack<StringPack>
{
  fn build(&self, requirements: &R) -> Result<StringValue, BuildError> {
    let next_requirements = requirements;
    let pack = next_requirements.pack()?;
    let operation = pack.operation();
    operation.build(requirements)
  }
}

#[derive(Debug)]
pub enum GeneralCreationValue {
  Empty {
    name: String,
  },
  Value {
    name: String,
    value: String,
  },
  Operation {
    name: String,
    elements: Vec<GeneralCreation>,
  },
}

impl CreationValue<StringOperation> for GeneralCreationValue {
  fn to_operation(self) -> StringOperation {
    match self {
      Self::Empty { .. } => StringOperation::Empty,
      Self::Value { value, .. } => StringValueOperation::new(value),
      Self::Operation { .. } => todo!(),
    }
  }
}

impl YaDeserialize for GeneralCreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreationValue = from_deserializer(reader)?;
    match inner {
      InnerGeneralCreationValue { name, value: Some(value), .. } =>
        Ok(Self::Value { name, value }),
      InnerGeneralCreationValue { name, elements, .. } if !elements.is_empty() =>
        Ok(Self::Operation { name, elements }),
      InnerGeneralCreationValue { name, .. } =>
        Ok(Self::Empty { name })
    }
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