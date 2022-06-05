#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter")]
pub struct InnerParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}