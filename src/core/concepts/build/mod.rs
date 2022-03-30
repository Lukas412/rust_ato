pub trait BuildableWithRequirements<Build, Requirements> {
  fn build_with_requirements(&self, requirements: Requirements) -> Build;
}
