pub trait Cache<T, E> {
  fn store(&mut self, name: String, value: T);
  fn get(&self, name: &String, load: &fn() -> Result<&T, E>) -> Result<&T, E>;
}