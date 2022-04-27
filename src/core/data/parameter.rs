use crate::core::data::variety::Variety;

pub struct Parameter {
  name: String,
  namespace: Option<String>,
  variety: Variety
}