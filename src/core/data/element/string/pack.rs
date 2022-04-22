use crate::core::data::element::argument::ElementArguments;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::element::string::operation::StringOperation;
use crate::core::data::element::parameter::ElementParameters;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;

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

impl File for StringPack {
  fn suffix() -> String {
    todo!()
  }
}

impl BuildableWithRequirements<StringElement, String, ElementArguments> for StringPack {
  fn build_with_requirements(&self, requirements: &ElementArguments) -> Result<StringElement, String> {
    todo!()
  }
}