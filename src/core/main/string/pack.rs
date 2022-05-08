use crate::ElementContainer;
use crate::core::main::build::BuildError;
use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
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

impl Buildable<StringValue, BuildError, ElementContainer> for StringPack
{
  fn build(&self, requirements: &ElementContainer) -> Result<StringValue, BuildError> {
    self.operation.build(requirements)
  }
}

impl Pack<StringValue, BuildError, ElementContainer> for StringPack
{
  fn namespace(&self) -> &String {
    &self.namespace
  }
}