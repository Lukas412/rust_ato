use crate::core::data::element::string::element::StringElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
pub struct StringGetArgumentOperation {
  name: String,
  namespace: String
}

impl BuildableWithRequirements<StringElement, String, Requirements> for StringGetArgumentOperation {
  fn build_with_requirements(&self, requirements: &Requirements) -> Result<StringElement, String> {
    todo!()
  }
}