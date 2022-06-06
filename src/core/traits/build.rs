use crate::core::build::error::BuildError;
use crate::{Creation, CreationStack};
use crate::core::main::general::pack::provider::PackProvider;

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> Creation;

  fn build(self, pack_provider: &PackProvider, requirements: &mut CreationStack) -> Result<B, BuildError>;
}