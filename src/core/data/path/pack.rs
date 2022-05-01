use crate::core::data::element::parameter::ElementParameters;
use crate::core::data::path::operation::PathOperation;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: PathOperation,
}