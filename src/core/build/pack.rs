use std::collections::HashMap;
use crate::core::build::error::BuildError;
use crate::core::traits::cache::Cache;
use crate::StringPack;

pub struct PackProvider {
  locations: Vec<String>,
  cache: PackCache,
}

pub struct PackCache {
  string_packs: HashMap<String, StringPack>,
}

impl Cache<StringPack, BuildError> for PackCache {
  fn store(&mut self, name: String, value: StringPack) {
    self.string_packs[name] = value
  }

  fn get(&mut self, name: &String, load: fn() -> Result<StringPack, BuildError>) -> Result<&StringPack, BuildError> {
    if let Some(value) = self.string_packs.get(name) {
      Ok(value)
    } else {
      match load() {
        Ok(value) => {
          self.store(name.to_owned(), value);
          &value
        }
        Err(error) => Err(error)
      }
    }
  }
}
