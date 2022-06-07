use std::rc::Rc;
use yaserde::YaDeserialize;
use std::io::Read;
use yaserde::de::Deserializer;
use crate::core::creation::InnerCreation;
use crate::core::operation::Operation;
use crate::core::parse::from_deserializer;

#[derive(Debug)]
pub struct CreationValue {
  name: String,
  operation: Operation,
}

impl CreationValue {
  pub fn to_name_and_operation(self) -> (String, Rc<Operation>) {
    (self.name, Rc::new(self.operation))
  }
}

impl CreationValue {
  fn from_inner(inner: InnerCreationValue) -> Self {
    let (name, operation) =
      match inner {
        InnerCreationValue { name, value: Some(value), .. } =>
          (name, Operation::Value(value)),
        InnerCreationValue { name, elements, .. } if !elements.is_empty() =>
          (name, Operation::Operation(elements)),
        InnerCreationValue { name, .. } =>
          (name, Operation::Empty)
      };
    Self { name, operation }
  }
}

impl YaDeserialize for CreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerCreationValue = from_deserializer(reader)?;
    Ok(Self::from_inner(inner))
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
  elements: Vec<InnerCreation>,
}
