use crate::core::build::error::BuildError;
use crate::core::main::element::container::ElementContainer;
use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::{Container, Provide};
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

impl<C> Buildable<PathValue, C> for PathPack
  where
    C: Container + Provide<PathValue, BuildError>
{
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    self.operation.build(requirements)
  }
}

impl Pack for PathPack {
  const SUFFIX: &'static str = "*.path.xml";

  fn namespace(&self) -> &String {
    &self.namespace
  }
}