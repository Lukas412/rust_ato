use crate::core::data::variety::Variety;

pub struct Parameters {
  parameters: Vec<Parameter>
}

pub struct Parameter {
  name: String,
  namespace: Option<String>,
  variety: Variety
}