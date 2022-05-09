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
  fn cache(&mut self) -> &mut HashMap<String, StringPack> {
    &mut self.string_packs
  }
}
