use crate::{Buildable, Container, ElementContainer};
use crate::core::data::build::BuildError;
use crate::core::data::element::parameter::ElementParameters;
use crate::core::data::path::operation::PathOperation;
use crate::core::data::path::value::PathValue;
use crate::core::traits::container::Provide;
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: PathOperation,
}

impl Buildable<PathValue, BuildError, ElementContainer> for PathPack
{
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    self.operation.build(requirements)
  }
}

impl Pack<PathValue, BuildError, ElementContainer> for PathPack
{
  fn namespace(&self) -> &String {
    &self.namespace
  }
}