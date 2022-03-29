pub trait Builder<Buildable, Build> {
  fn build(buildable: Buildable) -> Build;
}

pub trait BuilderWithRequirements<Buildable, Build, Requirements> {
  fn build_with_requirements(buildable: Buildable, requirements: Requirements) -> Build;
}