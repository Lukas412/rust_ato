pub trait BuildableWithRequirements<Build, Error, Requirements> {
  fn build_with_requirements(&self, requirements: Requirements) -> Result<Build, Error>;
}