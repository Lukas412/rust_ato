impl RequirementError {
  pub fn new(name: String, namespace: String) -> BuildError {
    BuildError::Requirement(Self {
      name,
      namespace,
      backtrace: Backtrace::default(),
    })
  }
}