use std::path::PathBuf;
use crate::core::build::error::BuildError;
use crate::core::main::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C> Buildable<PathValue, C> for PathValueOperation
  where
    C: Container + Provide<PathValue>
{
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    let value = PathBuf::from(&self.text);
    let namespace = requirements.namespace().to_owned();
    Ok(PathValue::new(value, namespace))
  }
}