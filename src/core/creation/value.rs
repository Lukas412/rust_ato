use yaserde::YaDeserialize;
use std::io::Read;
use yaserde::de::Deserializer;
use crate::core::operation::Operation;
use crate::core::variant::{DeserializeWithVariant, Variant};
use crate::Creation;

#[derive(Debug)]
pub struct CreationValue {
  name: String,
  operation: Operation,
}

impl CreationValue {
  pub fn to_name_and_operation(self) -> (String, Operation) {
    (self.name, self.operation)
  }
}

impl YaDeserialize for CreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    CreationValue::from_deserializer(reader)
  }
}

impl DeserializeWithVariant<InnerCreationValue> for CreationValue {
  fn from_inner(inner: InnerCreationValue, variant: Variant) -> Result<Self, String> {
    let (name, operation) = inner.to_name_and_rc_operation(variant);
    Ok(Self { name, operation })
  }
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
struct InnerCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "creation")]
  creation: Option<Creation>,
}

impl InnerCreationValue {
  fn to_name_and_rc_operation(self, variant: Variant) -> (String, Operation) {
    let (name, operation) =
      match self {
        InnerCreationValue { name, value: Some(value), .. } =>
          (name, Operation::new_value(value, variant)),
        InnerCreationValue { name, creation: Some(creation), .. } =>
          (name, Operation::new_creation(creation, variant)),
        InnerCreationValue { name, .. } =>
          (name, Operation::new_empty(variant))
      };
    (name, operation)
  }
}
