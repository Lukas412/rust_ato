use std::path::PathBuf;

use crate::core::build::error::BuildError;
use crate::core::main::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::value::Value;
use crate::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<PathValue> for PathValueOperation {
  fn build(&self, requirements: &Requirements) -> Result<PathValue, BuildError> {
    let value = PathBuf::from(&self.text);
    let namespace = requirements.get_owned_namespace();
    Ok(PathValue::new(value, namespace))
  }
}