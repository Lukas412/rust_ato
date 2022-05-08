impl ValueError {
  pub fn new(value: String, namespace: String) -> BuildError {
    BuildError::Value(Self {
      value,
      namespace,
      backtrace: Backtrace::default(),
    })
  }
}