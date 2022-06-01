use std::rc::Rc;
use yaserde::YaDeserialize;
use std::io::Read;
use yaserde::de::Deserializer;
use crate::core::main::general::creation::InnerGeneralCreation;
use crate::core::main::general::operation::GeneralOperation;
use crate::core::parse::from_deserializer;

#[derive(Debug)]
pub struct GeneralCreationValue {
  name: String,
  operation: GeneralOperation,
}

impl GeneralCreationValue {
  fn from_inner(inner: InnerGeneralCreationValue) -> Self {
    let (name, operation) =
      match inner {
        InnerGeneralCreationValue { name, value: Some(value), .. } =>
          (name, GeneralOperation::Value(value)),
        InnerGeneralCreationValue { name, elements, .. } if !elements.is_empty() =>
          (name, GeneralOperation::Operation(elements)),
        InnerGeneralCreationValue { name, .. } =>
          (name, GeneralOperation::Empty)
      };
    Self { name, operation }
  }

  pub fn to_name_and_operation(self) -> (String, Rc<GeneralOperation>) {
    (self.name, Rc::new(self.operation))
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
  elements: Vec<InnerGeneralCreation>,
}
