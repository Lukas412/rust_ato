use crate::core::data::element::boolean::parameter::BooleanParameter;
use crate::core::data::element::number::parameter::NumberParameter;
use crate::core::data::element::path::parameter::PathParameter;
use crate::core::data::element::string::parameter::StringParameter;

#[derive(Debug)]
pub struct Parameters(Vec<Parameter>);

impl Parameters {
  pub fn empty() -> Parameters {
    Parameters(vec![])
  }
}

#[derive(Debug)]
enum Parameter {
  Boolean(BooleanParameter),
  Number(NumberParameter),
  Path(PathParameter),
  String(StringParameter),
}