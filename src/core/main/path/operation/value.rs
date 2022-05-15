use std::path::PathBuf;
use crate::core::build::error::BuildError;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::path::value::PathValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::namespace::WithNamespace;
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<R> Buildable<PathValue, R> for PathValueOperation
  where
    R: WithNamespace + ProvideOperation<PathOperation>
{
  fn build(&self, requirements: &R) -> Result<PathValue, BuildError> {
    let value = PathBuf::from(&self.text);
    let namespace = requirements.get_owned_namespace();
    Ok(PathValue::new(value, namespace))
  }
}