use crate::Container;
use crate::core::data::build::BuildError;
use crate::core::data::element::container::ElementContainer;
use crate::core::data::element::parameter::ElementParameters;
use crate::core::data::string::operation::StringOperation;
use crate::core::data::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: StringOperation,
}

impl<C: Container> Pack<C> for StringPack {
  fn requirements(&self) -> C {
    C::new(self.namespace.to_owned())
  }
}

impl Buildable<StringValue, BuildError, ElementContainer> for StringPack {
  fn build(&self, requirements: &ElementContainer) -> Result<StringValue, BuildError> {
    todo!()
  }
}