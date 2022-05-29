use crate::core::main::general::parameter::GeneralParameters;
use crate::core::traits::namespace::Namespace;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::pack::Pack;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "pack", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringPack {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(child)]
  parameters: GeneralParameters,
  #[yaserde(flatten)]
  operation: StringOperation,
}

impl Pack<StringValue> for StringPack {
  type Operation = StringOperation;

  const SUFFIX: &'static str = ".string.xml";

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }

  fn operation(&self) -> &Self::Operation {
    &self.operation
  }
}