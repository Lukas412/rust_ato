use std::collections::HashMap;
use std::io::Read;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::main::general::reference::GeneralReferences;
use crate::core::main::path::pack::PathPack;
use crate::core::parse::from_deserializer;
use crate::core::traits::file::File;
use crate::StringPack;

#[derive(Debug, Default)]
pub struct GeneralBundle {
  namespace: String,
  location: String,
  path_packs: HashMap<String, PathPack>,
  string_packs: HashMap<String, StringPack>,
}

impl GeneralBundle {
  fn from_inner(inner: GeneralInnerBundle) -> Self {
    todo!()
  }
}

impl YaDeserialize for GeneralBundle {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let inner = from_deserializer(reader)?;
    Ok(Self::from_inner(inner))
  }
}

impl File for GeneralBundle {
  const SUFFIX: String = ".bundle.xml".to_owned();
}

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "bundle", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralInnerBundle {
  #[yaserde(attribute)]
  namespace: String,
  references: GeneralReferences,
}
