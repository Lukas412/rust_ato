use std::io::Read;
use std::rc::Rc;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::namespace::ParameterName;
use crate::Creation;
use crate::helpers::ser::events::start::peek_start_element;
use crate::helpers::ser::from::from_deserializer;

#[derive(Debug)]
pub(crate) enum OperationAction {
  Empty,
  Creation(Rc<Creation>),
  Value(String),
  GetArgument(ParameterName),
}

impl OperationAction {
  pub(crate) fn new_creation(creation: Rc<Creation>) -> Self {
    Self::Creation(creation)
  }

  pub(crate) fn new_empty() -> Self {
    Self::Empty
  }

  pub(crate) fn new_value(value: String) -> Self {
    Self::Value(value)
  }

  pub(crate) fn new_get_argument(name: ParameterName) -> Self {
    Self::GetArgument(name)
  }
}

impl OperationAction {
  fn new_value_from_reader<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerValueOperationAction = from_deserializer(reader)?;
    Ok(Self::new_value(inner.text))
  }

  fn new_get_argument_from_reader<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner: InnerGetArgumentOperationAction = from_deserializer(reader)?;
    let name = inner.to_parameter_name();
    Ok(Self::new_get_argument(name))
  }
}

impl YaDeserialize for OperationAction {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let (name, _, _) = peek_start_element(reader)?;
    match name.local_name.as_str() {
      "empty" => Ok(Self::new_empty()),
      "value" => Self::new_value_from_reader(reader),
      "get_argument" => Self::new_get_argument_from_reader(reader),
      name => Err(format!("No OperationAction named: {name}"))
    }
  }
}

impl Default for OperationAction {
  fn default() -> Self {
    Self::Empty
  }
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
struct InnerValueOperationAction {
  #[yaserde(text)]
  text: String,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument")]
struct InnerGetArgumentOperationAction {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(attribute)]
  name: String,
}

impl InnerGetArgumentOperationAction {
  fn to_parameter_name(self) -> ParameterName {
    ParameterName::new(self.namespace, self.name)
  }
}