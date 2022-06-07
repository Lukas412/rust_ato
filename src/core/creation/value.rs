use std::rc::Rc;
use yaserde::YaDeserialize;
use std::io::Read;
use yaserde::de::Deserializer;
use crate::core::creation::InnerCreation;
use crate::core::operation::Operation;
use crate::core::parse::from_deserializer;

#[derive(Debug)]
pub struct GeneralCreationValue {
  name: String,
  operation: Operation,
}

impl GeneralCreationValue {
  pub fn to_name_and_operation(self) -> (String, Rc<Operation>) {
    (self.name, Rc::new(self.operation))
  }
}

impl GeneralCreationValue {
  fn from_inner(inner: InnerGeneralCreationValue) -> Self {
    let (name, operation) =
      match inner {
        InnerGeneralCreationValue { name, value: Some(value), .. } =>
          (name, Operation::Value(value)),
        InnerGeneralCreationValue { name, elements, .. } if !elements.is_empty() =>
          (name, Operation::Operation(elements)),
        InnerGeneralCreationValue { name, .. } =>
          (name, Operation::Empty)
      };
    Self { name, operation }
  }
}

impl YaDeserialize for GeneralCreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGeneralCreationValue = from_deserializer(reader)?;
    Ok(Self::from_inner(inner))
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
  elements: Vec<InnerCreation>,
}
