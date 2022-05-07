use crate::core::data::build::BuildError;
use crate::core::data::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C: Provide<PathValue>> Buildable<PathValue, BuildError, C> for PathValueOperation {
  fn build(&self, _: &C) -> Result<PathValue, BuildError> {
    PathValue::from_str(&self.text)
  }
}