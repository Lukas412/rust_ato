use crate::core::main::general::value::GeneralValue;

#[derive(Debug, YaDeserialize)]
#[yaserde(root, rename = "main", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
pub struct GeneralCreation {
  uses: GeneralCreationUses,
  #[yaserde(prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
  element: GeneralCreationElement,
}

impl GeneralCreation {
  fn build(&self) -> GeneralValue {
    self.element.build()
  }
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "uses")]
pub struct GeneralCreationUses {
  #[yaserde(rename = "use")]
  uses: Vec<String>,
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
pub struct GeneralCreationElement {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
pub struct GeneralCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
  elements: Vec<GeneralCreationElement>,
}