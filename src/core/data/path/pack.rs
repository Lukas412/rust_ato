use crate::Container;
use crate::core::data::element::parameter::ElementParameters;
use crate::core::data::path::operation::PathOperation;
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

impl<C: Container> Pack<C> for PathPack {
  fn requirements(&self) -> C {
    C::new(self.namespace.to_owned())
  }
}