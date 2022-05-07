use std::path::PathBuf;
use crate::core::data::build::BuildError;
use crate::core::data::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C: Provide<PathValue>> Buildable<PathValue, BuildError, C> for PathValueOperation {
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    let value = PathBuf::from(&self.text);
    let namespace = requirements.namespace().to_owned();
    Ok(PathValue::new(value, namespace))
  }
}