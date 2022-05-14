use crate::core::build::error::BuildError;
use crate::core::main::general::container::GeneralContainer;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::creation::{Creation, CreationValue};
use crate::PackProvider;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "creation", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

impl Creation<StringValue> for GeneralCreation {
  type Container = GeneralContainer;
  type Value = GeneralCreationValue;

  fn namespace(&self) -> &String {
    todo!()
  }

  fn values<const N: usize>(&self) -> [(String, Self::Value); N] {
    todo!()
  }
}

impl Buildable<StringValue, PackProvider> for GeneralCreation {
  fn build(&self, requirements: &PackProvider) -> Result<StringValue, BuildError> {
    let pack = requirements.string_pack(&self.namespace)?;
    let container = self.container();
    pack.build(&container)
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

impl CreationValue for GeneralCreationValue {}