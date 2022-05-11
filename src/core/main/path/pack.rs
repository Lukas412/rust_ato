use crate::{Buildable, ElementContainer};
use crate::core::build::error::BuildError;
use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::path::value::PathValue;
use crate::core::traits::file::File;
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub struct PathPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: PathOperation,
}

impl File for PathPack {
  const SUFFIX: &'static str = "*.path.xml";
}

impl Buildable<PathValue, BuildError, ElementContainer> for PathPack
{
  fn build(&self, requirements: &ElementContainer) -> Result<PathValue, BuildError> {
    self.operation.build(requirements)
  }
}

impl Pack<PathValue, BuildError, ElementContainer> for PathPack
{
  fn namespace(&self) -> &String {
    &self.namespace
  }
}