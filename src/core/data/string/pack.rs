use crate::core::data::build::BuildError;
use crate::core::data::parameter::Parameters;
use crate::core::data::string::element::StringElement;
use crate::core::data::string::operation::StringOperation;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: Parameters,
  #[yaserde(flatten)]
  operation: StringOperation,
}