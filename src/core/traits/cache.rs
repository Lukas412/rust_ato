pub trait Cache<T, E> {
  fn cache<M: Map>(&mut self) -> &mut M;
  fn store(&mut self, name: String, value: T) {
    self.cache()[name] = value
  }
  fn get(&mut self, name: &String, load: &fn() -> Result<&T, E>) -> Result<&T, E> {
    let cached_instance = self.cache().get(name);
    if let Some(value) = cached_instance {
      Ok(value)
    } else {
      let loaded_instance = load();
      match loaded_instance {
        Ok(value) => {
          self.store(name.to_owned(), value);
          &value
        }
        Err(error) => Err(error)
      }
    }
  }
}