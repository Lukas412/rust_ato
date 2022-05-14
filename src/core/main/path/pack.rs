use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::namespace::Namespace;
use crate::core::main::path::operation::PathOperation;
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

impl Pack for PathPack {
  const SUFFIX: &'static str = "*.path.xml";

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }
}