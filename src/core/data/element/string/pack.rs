use std::io::Read;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::parameter::Parameters;
use crate::core::data::requirement::Requirements;
use crate::core::namespace::Namespace;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;

struct StringPack {
  namespace: Namespace,
  parameters: Parameters,
  operation: Box<dyn Operation<StringElement, String>>,
}

impl File for StringPack {
  fn suffix() -> String {
    todo!()
  }
}

impl YaDeserialize for StringPack {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    todo!()
  }
}

impl BuildableWithRequirements<StringElement, Requirements> for StringPack {
  fn build_with_requirements(&self, requirements: Requirements) -> StringElement {
    todo!()
  }
}

impl Pack<StringElement, String> for StringPack {}