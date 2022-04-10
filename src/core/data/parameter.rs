use crate::core::data::element::boolean::parameter::BooleanParameter;
use crate::core::data::element::number::parameter::NumberParameter;
use crate::core::data::element::path::parameter::PathParameter;
use crate::core::data::element::string::parameter::StringParameter;

pub struct Parameters(Vec<Parameter>);

#[derive(Debug, YaDeserialize)]
enum Parameter {
  Boolean(BooleanParameter),
  Number(NumberParameter),
  Path(PathParameter),
  String(StringParameter),
}