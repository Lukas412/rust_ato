#[derive(Debug, Default, YaDeserialize)]
pub struct Parameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<Parameter>
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter")]
pub struct Parameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}