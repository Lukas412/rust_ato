use crate::core::data::element::parameter::ElementParameters;
use crate::core::data::string::operation::StringOperation;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: StringOperation,
}