use std::rc::Rc;
use yaserde::YaDeserialize;
use std::io::Read;
use yaserde::de::Deserializer;
use crate::core::operation::Operation;
use crate::core::parse::from_deserializer;
use crate::core::variant::Variant;
use crate::Creation;

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
  fn from_inner(inner: InnerCreationValue, variant: Variant) -> Self {
    let (name, operation) =
      match inner {
        InnerCreationValue { name, value: Some(value), .. } =>
          (name, Operation::new_value(value, variant)),
        InnerCreationValue { name, creation: Some(creation), .. } =>
          (name, Operation::new_creation(creation, variant)),
        InnerCreationValue { name, .. } =>
          (name, Operation::new_empty(variant))
      };
    Self { name, operation }
  }
}

impl YaDeserialize for CreationValue {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let variant = Variant::from_owned_name();
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
  creation: Option<Creation>,
}
