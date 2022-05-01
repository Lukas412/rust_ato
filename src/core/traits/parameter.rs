use crate::core::data::name::AtoName;

pub trait Parameter {
  fn name(&self) -> &AtoName;
}