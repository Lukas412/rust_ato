use crate::core::build::error::BuildError;
use crate::core::main::general::container::GeneralContainer;
use crate::core::main::namespace::Namespace;
use crate::core::main::string::pack::StringPack;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::creation::{Creation, CreationValue};
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::ProvidePack;
use crate::GeneralPackProvider;

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
  type Pack = StringPack;
  type Value = GeneralCreationValue;

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }

  fn values(&self) -> Vec<(String, Self::Value)> {
    Vec::new()
  }
}

impl Buildable<StringValue, GeneralPackProvider> for GeneralCreation
{
  fn build(&self, requirements: &GeneralPackProvider) -> Result<StringValue, BuildError> {
    let pack: &StringPack = requirements.pack(self.namespace())?;
    let container = self.container(requirements)?;
    todo!("{:?} {:?}", pack, container)
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

impl CreationValue<StringValue> for GeneralCreationValue
{
  fn operation<O: Operation>(&self) -> O {
    todo!()
  }
}