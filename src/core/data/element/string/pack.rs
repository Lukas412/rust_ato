use std::io::Read;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::element::string::operation::StringOperation;
use crate::core::data::parameter::Parameters;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: Option<Parameters>,
  #[yaserde(flatten)]
  operation: StringOperation,
}

impl File for StringPack {
  fn suffix() -> String {
    todo!()
  }
}

impl BuildableWithRequirements<StringElement, String, Requirements> for StringPack {
  fn build_with_requirements(&self, requirements: Requirements) -> Result<StringElement, String> {
    todo!()
  }
}

impl Pack<StringElement, String> for StringPack {}