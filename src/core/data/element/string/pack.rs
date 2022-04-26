use crate::core::data::build::BuildError;
use crate::core::data::element::creation::ElementCreation;
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

impl BuildableWithRequirements<StringElement, BuildError, ElementCreation> for StringPack {
  fn build_with_requirements(&self, requirements: &ElementCreation) -> Result<StringElement, BuildError> {
    self.operation.build_with_requirements(requirements)
  }
}