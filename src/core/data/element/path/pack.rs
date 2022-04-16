use crate::core::data::element::path::operation::PathOperation;
use crate::core::data::parameter::Parameters;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: Parameters,
  #[yaserde(flatten)]
  operation: PathOperation,
}