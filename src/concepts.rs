pub trait Buildable<Build> {
  fn build(&self) -> Build;
}

pub trait BuilderWithRequirements<Buildable, Build, Requirements> {
  fn build_with_requirements(buildable: Buildable, requirements: Requirements) -> Build;
}