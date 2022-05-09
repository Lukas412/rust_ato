#[derive(Debug, YaDeserialize)]
#[yaserde(root, rename = "main", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
pub struct Creation {
  uses: CreationUses,
  #[yaserde(prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
  element: CreationElement,
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "uses")]
pub struct CreationUses {
  #[yaserde(rename = "use")]
  uses: Vec<String>,
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
pub struct CreationElement {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<CreationValue>,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
pub struct CreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
  elements: Vec<CreationElement>,
}