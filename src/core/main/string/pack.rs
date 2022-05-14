use crate::core::main::element::parameter::ElementParameters;
use crate::core::main::namespace::Namespace;
use crate::core::main::string::operation::StringOperation;
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

impl Pack for StringPack {
  const SUFFIX: &'static str = ".string.xml";

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }
}