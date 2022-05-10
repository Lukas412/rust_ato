use crate::core::main::general::reference::GeneralReferences;
use crate::core::traits::file::File;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "bundle", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralBundle {
  #[yaserde(attribute)]
  namespace: String,
  references: GeneralReferences,
}

impl File for GeneralBundle {
  fn suffix() -> String {
    ".bundle.xml".to_owned()
  }
}