use crate::core::data::element::boolean::parameter::BooleanParameter;
use crate::core::data::element::number::parameter::NumberParameter;
use crate::core::data::element::path::parameter::PathParameter;
use crate::core::data::element::string::parameter::StringParameter;

#[derive(Debug, Default, YaDeserialize)]
pub struct Parameters {
  #[yaserde(rename = "parameter", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
  boolean: Vec<BooleanParameter>,
  #[yaserde(rename = "parameter", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
  number: Vec<NumberParameter>,
  #[yaserde(rename = "parameter", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  path: Vec<PathParameter>,
  #[yaserde(rename = "parameter", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  string: Vec<StringParameter>
}