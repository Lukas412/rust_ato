use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::path::value::PathValue;
use crate::core::traits::build::Buildable;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<PathValue, BuildError, ElementCreation> for PathValueOperation {
  fn build(&self, _: &ElementCreation) -> Result<PathValue, BuildError> {
    PathValue::from_str(&self.text)
  }
}