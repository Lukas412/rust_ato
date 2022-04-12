use crate::core::data::element::boolean::parameter::BooleanParameter;
use crate::core::data::element::number::parameter::NumberParameter;
use crate::core::data::element::path::parameter::PathParameter;
use crate::core::data::element::string::parameter::StringParameter;

#[derive(Debug)]
pub struct Parameters(Vec<Parameter>);

impl Default for Parameters {
  fn default() -> Self {
    Parameters(vec![])
  }
}

#[derive(Debug, YaDeserialize)]
enum Parameter {
  #[yaserde(rename = "parameter", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
  Boolean {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    namespace: Option<String>,
  },
  #[yaserde(rename = "parameter", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
  Number {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    namespace: Option<String>,
  },#[yaserde(rename = "parameter", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  Path {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    namespace: Option<String>,
  },#[yaserde(rename = "parameter", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  String {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    namespace: Option<String>,
  }
}