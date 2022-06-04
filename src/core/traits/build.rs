use crate::core::build::error::BuildError;
use crate::{GeneralCreation, PackProvider, GeneralCreationStack};

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> GeneralCreation;

  fn build(self, pack_provider: &PackProvider, requirements: &mut GeneralCreationStack) -> Result<B, BuildError>;
}