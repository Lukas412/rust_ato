use crate::core::build::error::BuildError;
use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: ElementParameters,
  #[yaserde(flatten)]
  operation: StringOperation,
}

impl<C> Buildable<StringValue, C> for StringPack
  where
    C: Container + Provide<StringValue>
{
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    self.operation.build(requirements)
  }
}

impl Pack for StringPack {
  const SUFFIX: &'static str = ".string.xml";

  fn namespace(&self) -> &String {
    &self.namespace
  }
}