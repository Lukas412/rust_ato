use crate::core::build::error::BuildError;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::PackProvider;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

impl Buildable<StringValue, BuildError, PackProvider> for GeneralCreation {
  fn build(&self, requirements: &PackProvider) -> Result<StringValue, BuildError> {
    let pack = requirements.string_pack(&self.namespace)?;
    let container = todo!();
    pack.build(container)
  }
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
pub struct GeneralCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
  elements: Vec<GeneralCreation>,
}